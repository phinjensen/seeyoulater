use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Bookmark {
    id: i64,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub fn initialize() -> Result<()> {
    let conn = Connection::open("./seeyoulater.db")?;
    conn.execute(
        "CREATE TABLE bookmark (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            url         TEXT UNIQUE,
            title       TEXT,
            description TEXT
            )",
        (),
    )?;
    Ok(())
}

pub fn add_bookmark(
    url: String,
    title: &Option<String>,
    description: &Option<String>,
) -> Result<Bookmark> {
    let conn = Connection::open("./seeyoulater.db")?;
    let mut stmt =
        conn.prepare("SELECT id, url, title, description FROM bookmark WHERE url = ?")?;
    let mut bookmarks = stmt.query_map([&url], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
        })
    })?;
    if let Some(Ok(bookmark)) = bookmarks.next() {
        println!("A bookmark for that URL already exists:");
        Ok(bookmark)
    } else {
        conn.execute(
            "INSERT INTO bookmark (url, title, description) VALUES (?, ?, ?)",
            (&url, title, description),
        )?;
        Ok(Bookmark {
            id: conn.last_insert_rowid(),
            url,
            title: title.to_owned(),
            description: description.to_owned(),
        })
    }
}
