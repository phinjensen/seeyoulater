use core::fmt;
use std::fmt::{Display, Formatter};

use rusqlite::{Connection, Result, ToSql, Transaction};

use crate::{migrations::MIGRATIONS, web::Metadata};

#[derive(Debug)]
pub struct Bookmark {
    id: i64,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

const CURRENT_VERSION: usize = 0;

impl Bookmark {
    fn format_tags(&self) -> String {
        if self.tags.len() > 0 {
            format!("[\x1b[33m{}\x1b[m]", &self.tags.join("\x1b[m,\x1b[33m"))
        } else {
            String::from("")
        }
    }

    fn write_url(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[36m{}\x1b[m", &self.url)
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(title) = &self.title {
            // TODO: Use a cross-platform library for term colors. Or roll our own, if most have
            // too many dependencies
            write!(f, "\x1b[1;32m{}\x1b[m", title)?;
        } else {
            write!(f, "\x1b[1;32m{}\x1b[m", &self.url)?;
        }
        writeln!(f, " {}", self.format_tags())?;
        self.write_url(f)?;
        if let Some(description) = &self.description {
            write!(f, "\n{}", description)?;
        };
        Ok(())
    }
}

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;
        let db = Database { connection };
        match db.connection.query_row(
            "SELECT value FROM syl_meta WHERE key='database_version'",
            (),
            |row| row.get::<usize, String>(0),
        ) {
            Ok(db_version) => {
                if let Ok(version) = db_version.parse::<usize>() {
                    db.migrate(version)?;
                } else {
                    panic!("Error parsing database version number! Your database may be corrupt.");
                }
            }
            Err(_) => {
                db.initialize()?;
            }
        }
        Ok(db)
    }

    pub fn initialize(&self) -> Result<()> {
        eprintln!("Initializing database...");
        self.connection.execute_batch(
            format!(
                "
            BEGIN;
            CREATE TABLE bookmark (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                url             TEXT UNIQUE,
                title           TEXT,
                description     TEXT,
                created_at      INTEGER
            );
            CREATE TABLE tag (
                name            TEXT PRIMARY KEY
            );
            CREATE TABLE bookmark_tag (
                bookmark_id     INTEGER REFERENCES bookmark (id),
                tag_name        TEXT REFERENCES tag (name)
            );
            CREATE TABLE syl_meta (
                key             TEXT UNIQUE,
                value           TEXT
            );
            INSERT INTO syl_meta VALUES ('database_version', {});
            COMMIT;
            ",
                CURRENT_VERSION
            )
            .as_str(),
        )?;
        self.migrate(CURRENT_VERSION)?;
        Ok(())
    }

    pub fn add_bookmark(
        &mut self,
        url: &String,
        metadata: Metadata,
        tags: &Vec<String>,
    ) -> Result<Bookmark> {
        let tx = self.connection.transaction()?;
        let bookmark = tx.query_row(
            "SELECT id, url, title, description, group_concat(tag_name)
            FROM bookmark
            LEFT JOIN bookmark_tag ON bookmark_tag.bookmark_id = bookmark.id
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

    pub fn search_bookmarks(
        &self,
        query: &Option<String>,
        tags: &Vec<String>,
    ) -> Result<Vec<Bookmark>> {
        // TODO: Come up with some ranking/ordering. Perhaps:
        // https://www.sqlite.org/fts3.html
        // TODO: Make search queries optional, to allow searching by tag only
        let mut select = String::from(
            "SELECT id, url, title, description, group_concat(tag_name)
            FROM bookmark
            LEFT JOIN bookmark_tag ON bookmark_tag.bookmark_id = bookmark.id",
        );
        let mut params: Vec<&dyn ToSql> = Vec::new();
        if let Some(query) = query {
            select += " WHERE (
                url LIKE '%' || ? || '%'
                OR title LIKE '%' || ? || '%'
                OR description LIKE '%' || ? || '%'
            )";
            params.extend_from_slice(&[query, query, query]);
        }
        if tags.len() > 0 {
            select = select
                + &format!(
                    " AND id IN (SELECT bookmark_id FROM bookmark_tag WHERE tag_name IN ({}))",
                    &"?,".repeat(tags.len())[..tags.len() * 2 - 1]
                );
            for tag in tags {
                params.push(tag as &dyn ToSql);
            }
        }
        select += &" GROUP BY id";
        let mut stmt = self.connection.prepare(&select)?;
        let bookmarks = stmt
            .query_map(&params[..], |row| {
                Ok(Bookmark {
                    id: row.get(0)?,
                    url: row.get(1)?,
                    title: row.get(2)?,
                    description: row.get(3)?,
                    tags: if let Some(s) = row.get::<_, Option<String>>(4)? {
                        s.split(',').map(|t| t.to_string()).collect()
                    } else {
                        Vec::new()
                    },
                })
            })?
            .map(|b| b.unwrap())
            .collect();
        Ok(bookmarks)
    }

    pub fn migrate(&self, current_version: usize) -> Result<()> {
        if current_version < MIGRATIONS.len() {
            for i in current_version..MIGRATIONS.len() {
                eprintln!("Migrating database to version {}", i + 1);
                self.connection.execute_batch(MIGRATIONS[i])?;
            }
        }
        Ok(())
    }
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
