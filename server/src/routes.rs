use rouille::input::json_input;
use rouille::{try_or_400, Request, Response};
use serde::Serialize;
use syl_lib::commands::{Add, Search, Tags};
use syl_lib::db::Database;
use syl_lib::web::{get_metadata, Metadata};

#[derive(Serialize)]
struct Error {
    message: String,
}

pub fn add(db: &mut Database, request: &Request) -> Response {
    let args: Add = try_or_400!(json_input(request));
    // TODO: Better handling of metadata. i.e. when both title and description are provided, don't
    // do a fetch. When one is provided but not the other, use the explicit one and do a fetch for
    // the other. When neither are provided, do a fetch. Also add an option that forces no fetch to
    // happen.
    let metadata = if let Some(title) = args.title {
        Metadata {
            title: Some(title),
            description: None,
        }
    } else {
        get_metadata(&args.url).unwrap_or(Metadata {
            title: None,
            description: None,
        })
    };
    match db.add_bookmark(&args.url, metadata, &args.tags) {
        Ok(bookmark) => Response::json(&bookmark),
        Err(e) => Response::json(&Error {
            message: format!("Error writing bookmark to database: {e}"),
        }),
    }
}

pub fn search(db: &mut Database, request: &Request) -> Response {
    let args: Search = serde_qs::from_str(request.raw_query_string()).unwrap();
    match db.search_bookmarks(&args.query, &args.tags, args.all_tags) {
        Ok(bookmarks) => Response::json(&bookmarks),
        Err(e) => Response::json(&Error {
            message: format!("Error searching bookmarks: {e}"),
        }),
    }
}

pub fn delete(db: &mut Database, request: &Request) -> Response {
    let args: Search = serde_qs::from_str(request.raw_query_string()).unwrap();
    match db.search_bookmarks(&args.query, &args.tags, args.all_tags) {
        Ok(bookmarks) => match db.delete_bookmarks(bookmarks.iter().map(|b| b.id).collect()) {
            Ok(deleted) => Response::json(&deleted),
            Err(e) => Response::json(&Error {
                message: format!("Error deleting bookmarks: {e}"),
            }),
        },
        Err(e) => Response::json(&Error {
            message: format!("Error searching bookmarks: {e}"),
        }),
    }
}

pub fn tags(db: &mut Database, request: &Request) -> Response {
    let args: Tags = serde_qs::from_str(request.raw_query_string()).unwrap();
    match db.get_tags(args.sort_by_count, args.reverse) {
        Ok(tags) => Response::json(&tags),
        Err(e) => Response::json(&Error {
            message: format!("Error searching bookmarks: {e}"),
        }),
    }
}
