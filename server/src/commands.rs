use syl_lib::{
    commands::{Add, Delete, Interface, Search, Tags},
    db::Database,
    web::{get_metadata, Metadata},
};

pub struct DatabaseInterface {
    db: Database,
}

impl DatabaseInterface {
    pub fn from(db: Database) -> Self {
        Self { db }
    }
}

impl Interface for DatabaseInterface {
    fn add(&mut self, args: Add) {
        match self.db.add_bookmark(
            &args.url,
            get_metadata(&args.url).unwrap_or(Metadata {
                title: None,
                description: None,
            }),
            &args.tags,
        ) {
            Ok(bookmark) => println!("{}", bookmark),
            Err(e) => eprintln!("Error adding bookmark to database: {:?}", e),
        }
    }

    fn find(&self, args: Search) {
        panic!("TODO");
    }

    fn tags(&self, args: Tags) {
        panic!("TODO");
    }

    fn delete(&self, args: Delete) {
        panic!("TODO");
    }
}
