use core::fmt;
use std::fmt::{Display, Formatter};

use itertools::Itertools;
use rusqlite::{params_from_iter, Connection, Result, Row, ToSql, Transaction};
use serde::{Deserialize, Serialize};

use crate::{
    colors::{color, Color},
    migrations::MIGRATIONS,
    web::Metadata,
};

pub use rusqlite::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: i64,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

const CURRENT_VERSION: usize = 0;

impl Bookmark {
    fn from_row(row: &Row<'_>) -> Result<Self> {
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
    }

    fn format_tags(&self) -> String {
        if self.tags.len() > 0 {
            format!(
                "[{}]",
                &self
                    .tags
                    .iter()
                    .map(|t| color(t, Color::Yellow))
                    .intersperse(",".to_string())
                    .collect::<String>()
            )
        } else {
            String::from("")
        }
    }

    fn write_url(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", color(&self.url, Color::Cyan))
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", color(&self.id.to_string(), Color::Blue))?;
        if let Some(title) = &self.title {
            write!(f, " {}", color(title, Color::BoldGreen))?;
        } else {
            write!(f, " {}", color(&self.url, Color::BoldGreen))?;
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
                    tag_name        TEXT REFERENCES tag (name),
                    PRIMARY KEY (bookmark_id, tag_name)
                );
                CREATE TABLE syl_meta (
                    key             TEXT PRIMARY KEY,
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
        url: &str,
        metadata: Metadata,
        tags: &Vec<String>,
    ) -> Result<Bookmark> {
        let tx = self.connection.transaction()?;
        let bookmark = tx.query_row(
            "
            SELECT id, url, title, description, group_concat(tag_name)
            FROM bookmark
            LEFT JOIN bookmark_tag ON bookmark_tag.bookmark_id = bookmark.id
            WHERE url = ?
            GROUP BY tag_name
            ",
            [url],
            Bookmark::from_row,
        );
        match bookmark {
            Ok(bookmark) => {
                println!("A bookmark for that URL already exists:");
                Ok(bookmark)
            }
            Err(Error::QueryReturnedNoRows) => {
                println!("Added bookmark:");
                tx.execute(
                    "INSERT INTO bookmark (url, title, description, created_at)
                    VALUES (?, ?, ?, datetime('now'))",
                    (url, &metadata.title, &metadata.description),
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
            Err(e) => {
                panic!("Error adding new bookmark: {}", e);
            }
        }
    }

    pub fn search_bookmarks(
        &self,
        query: &Option<String>,
        tags: &Vec<String>,
        all_tags: bool,
    ) -> Result<Vec<Bookmark>> {
        // TODO: Come up with some ranking/ordering. Perhaps:
        // https://www.sqlite.org/fts3.html
        let mut select = String::from(
            "SELECT id, url, title, description, group_concat(tag_name)
            FROM bookmark
            LEFT JOIN bookmark_tag ON bookmark_tag.bookmark_id = bookmark.id
            WHERE 1",
        );
        let mut params: Vec<&dyn ToSql> = Vec::new();
        if let Some(query) = query {
            select += " AND (
                url LIKE '%' || ? || '%'
                OR title LIKE '%' || ? || '%'
                OR description LIKE '%' || ? || '%'
            )";
            params.extend_from_slice(&[query, query, query]);
        }
        if tags.len() > 0 {
            select += &format!(
                " AND id IN (SELECT bookmark_id FROM bookmark_tag WHERE tag_name IN ({})",
                repeat_vars(tags.len()),
            );
            if all_tags {
                select += &format!(
                    " GROUP BY bookmark_id HAVING count(bookmark_id) = {}",
                    tags.len()
                );
            }
            select += ")";
            for tag in tags {
                params.push(tag as &dyn ToSql);
            }
        }
        select += " GROUP BY id";
        let mut stmt = self.connection.prepare(&select)?;
        let bookmarks = stmt
            .query_map(&params[..], Bookmark::from_row)?
            .map(|b| b.unwrap())
            .collect();
        Ok(bookmarks)
    }

    pub fn rename_tag(&self, from: &str, to: &str) -> Result<usize> {
        let mut stmt = self
            .connection
            .prepare("SELECT count(name) FROM tag WHERE name = ?")?;
        let found: isize = stmt.query_row(&[to], |r| r.get(0))?;
        if found == 0 {
            self.connection
                .execute("INSERT INTO tag VALUES (?)", &[to])?;
        }
        self.connection.execute(
            "UPDATE bookmark_tag SET tag_name = ? WHERE tag_name = ?",
            &[to, from],
        )
    }

    pub fn get_tags(&self, sort_by_count: bool, reverse: bool) -> Result<Vec<(String, usize)>> {
        let mut stmt = self.connection.prepare(
            format!(
                "
                SELECT name, count(bookmark_id) as count
                FROM tag
                JOIN bookmark_tag ON bookmark_tag.tag_name=name
                GROUP BY name
                ORDER BY {} {}
                ",
                if sort_by_count { "count" } else { "name" },
                if reverse { "DESC" } else { "ASC" },
            )
            .as_str(),
        )?;
        let tags = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, usize>(1)?))
            })?
            .map(|t| t.unwrap())
            .collect();
        Ok(tags)
    }

    pub fn delete_bookmarks(&self, ids: Vec<i64>) -> Result<usize> {
        self.connection.execute(
            &format!(
                "DELETE FROM bookmark_tag WHERE bookmark_id IN ({})",
                repeat_vars(ids.len())
            ),
            params_from_iter(&ids),
        )?;
        self.connection.execute(
            &format!(
                "DELETE FROM bookmark WHERE id IN ({})",
                repeat_vars(ids.len())
            ),
            params_from_iter(&ids),
        )
    }

    fn migrate(&self, current_version: usize) -> Result<()> {
        if current_version < MIGRATIONS.len() {
            for i in current_version..MIGRATIONS.len() {
                eprintln!("Migrating database to version {}", i + 1);
                self.connection.execute_batch(MIGRATIONS[i])?;
            }
        }
        Ok(())
    }
}

// TODO: Consider moving away from static. Am I right in understanding that creating this as a
// static-lifetime object will make it stay until the program exits? Would that increase the
// overall memory of the server every time this function is called without cleaning it up?
fn repeat_vars(times: usize) -> String {
    "?,".repeat(times)[..times * 2 - 1].to_string()
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    fn get_db() -> rusqlite::Result<Database> {
        Database::open(":memory:")
    }

    fn add_bookmark_no_meta(
        db: &mut Database,
        url: &str,
        tags: &Vec<String>,
    ) -> rusqlite::Result<Bookmark> {
        db.add_bookmark(
            url,
            Metadata {
                title: None,
                description: None,
            },
            tags,
        )
    }

    // rename_tag should create a new tag if necessary
    #[test]
    fn test_rename_tag_new() -> Result<(), Box<dyn Error>> {
        let mut db = get_db()?;
        add_bookmark_no_meta(&mut db, &"example.org/one", &vec!["tag-one".to_string()])?;

        let tags = db.get_tags(true, false)?;
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].0, "tag-one");
        db.rename_tag("tag-one", "tag-three")?;
        let tags = db.get_tags(true, false)?;
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].0, "tag-three");
        Ok(())
    }

    #[test]
    fn test_rename_tag_existing() -> Result<(), Box<dyn Error>> {
        let mut db = get_db()?;
        add_bookmark_no_meta(&mut db, &"example.org/one", &vec!["tag-one".to_string()])?;
        add_bookmark_no_meta(&mut db, &"example.org/two", &vec!["tag-two".to_string()])?;

        let tags = db.get_tags(true, false)?;
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].1, 1);
        assert_eq!(tags[1].1, 1);
        db.rename_tag("tag-one", "tag-two")?;
        let tags = db.get_tags(true, false)?;
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].0, "tag-two");
        assert_eq!(tags[0].1, 2);

        Ok(())
    }

    #[test]
    fn test_rename_tag_nonexistent() -> Result<(), Box<dyn Error>> {
        let mut db = get_db()?;
        db.rename_tag("tag-one", "tag-two")?;
        let tags = db.get_tags(true, false)?;
        assert_eq!(tags.len(), 0);

        add_bookmark_no_meta(&mut db, &"example.org/two", &vec!["tag-two".to_string()])?;
        db.rename_tag("tag-one", "tag-two")?;
        let tags = db.get_tags(true, false)?;
        assert_eq!(tags.len(), 1);

        Ok(())
    }
}
