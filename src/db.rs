use core::fmt;
use std::fmt::{Display, Formatter};

use rusqlite::{Connection, Result, Transaction};

use crate::web::Metadata;

#[derive(Debug)]
pub struct Bookmark {
    id: i64,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut result;
        if let Some(title) = &self.title {
            writeln!(
                f,
                "\x1b[1;32m{}\x1b[m [\x1b[33m{}\x1b[m]",
                title,
                &self.tags.join("\x1b[m,\x1b[33m")
            )?;
        };
        result = write!(f, "\x1b[36m{}\x1b[m", &self.url);
        if let Some(description) = &self.description {
            result = write!(f, "\n{}", description);
        };
        result
    }
}

pub fn initialize() -> Result<()> {
    let conn = Connection::open("./seeyoulater.db")?;
    conn.execute(
        "CREATE TABLE bookmark (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            url         TEXT UNIQUE,
            title       TEXT,
            description TEXT,
            created_at  INTEGER
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE tag (
            name        TEXT PRIMARY KEY
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE bookmark_tag (
            bookmark_id     INTEGER REFERENCES bookmark (id),
            tag_name        TEXT REFERENCES tag (name)
        )",
        (),
    )?;
    Ok(())
}

fn add_tags(tx: &Transaction, id: i64, tags: &Vec<String>) -> Result<()> {
    let mut tag_insert = tx.prepare("INSERT OR IGNORE INTO tag VALUES (?)")?;
    for tag in tags {
        tag_insert.execute([&tag])?;
    }
    let mut bookmark_tag_insert =
        tx.prepare("INSERT OR IGNORE INTO bookmark_tag (bookmark_id, tag_name) VALUES (?, ?)")?;
    for tag in tags {
        bookmark_tag_insert.execute((id, tag))?;
    }
    Ok(())
}

pub fn add_bookmark(url: &String, metadata: Metadata, tags: &Vec<String>) -> Result<Bookmark> {
    let mut conn = Connection::open("./seeyoulater.db")?;
    let tx = conn.transaction()?;
    let bookmark = tx.query_row(
        "SELECT id, url, title, description, group_concat(tag_name)
            FROM bookmark
            JOIN bookmark_tag ON bookmark_tag.bookmark_id = bookmark.id
            WHERE url = ?",
        [&url],
        |row| {
            Ok(Bookmark {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                tags: row
                    .get::<_, String>(4)?
                    .split(',')
                    .map(|t| t.to_string())
                    .collect(),
            })
        },
    );
    if let Ok(bookmark) = bookmark {
        println!("A bookmark for that URL already exists:");
        Ok(bookmark)
    } else {
        println!("Added bookmark:");
        tx.execute(
            "INSERT INTO bookmark (url, title, description, created_at)
                VALUES (?, ?, ?, datetime('now'))",
            (&url, &metadata.title, &metadata.description),
        )?;
        let id = tx.last_insert_rowid();
        add_tags(&tx, id, tags)?;
        tx.commit()?;
        Ok(Bookmark {
            id,
            url: url.to_string(),
            title: metadata.title,
            description: metadata.description,
            tags: tags.to_vec(),
        })
    }
}

pub fn search_bookmarks(query: &String) -> Result<Vec<Bookmark>> {
    let conn = Connection::open("./seeyoulater.db")?;
    let mut stmt = conn.prepare(
        "SELECT id, url, title, description, group_concat(tag_name)
            FROM bookmark
            JOIN bookmark_tag ON bookmark_tag.bookmark_id = bookmark.id
            WHERE url LIKE '%' || ? || '%'",
    )?;
    let bookmarks = stmt
        .query_map([&query], |row| {
            Ok(Bookmark {
                id: row.get(0)?,
                url: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                tags: row
                    .get::<_, String>(4)?
                    .split(',')
                    .map(|t| t.to_string())
                    .collect(),
            })
        })?
        .map(|b| b.unwrap())
        .collect();
    Ok(bookmarks)
}
