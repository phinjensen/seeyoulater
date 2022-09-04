#[macro_use]
extern crate rouille;

use std::io;
use std::sync::Mutex;
use syl_lib::config::Config;
use syl_lib::db::Database;
use syl_server::routes::{add, search, tags};

const PORT: usize = 8080;

fn main() {
    // This example demonstrates how to handle HTML forms.

    // Note that like all examples we only listen on `localhost`, so you can't access this server
    // from another machine than your own.
    println!("Now listening on localhost:{PORT}");

    let config = Config::new();
    let db = Mutex::new(Database::open(&config.database()).unwrap());

    rouille::start_server(format!("localhost:{PORT}"), move |request| {
        rouille::log(&request, io::stdout(), || {
            router!(request,
                (POST) (/add) => {
                    add(&mut db.lock().unwrap(), request)
                },
                (GET) (/search) => {
                    search(&mut db.lock().unwrap(), request)
                },
                (DELETE) (/search) => {
                    rouille::Response::text("Deleting bookmarks on the server is not yet supported.").with_status_code(501)
                },
                (GET) (/tags) => {
                    tags(&mut db.lock().unwrap(), request)
                },
                _ => rouille::Response::empty_404()
            )
        })
    });
}
