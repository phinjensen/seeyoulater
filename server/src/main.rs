#[macro_use]
extern crate rouille;

use std::io;
use std::sync::Mutex;
use syl_lib::commands::DatabaseInterface;
use syl_lib::config::{Config, ConfigPath};
use syl_lib::db::Database;
use syl_lib::web::WebClient;
use syl_server::routes::{add, delete, search, tags};

fn main() {
    let config = Config::open(ConfigPath::ServerDefault);

    match &config.server {
        Some(server) => println!("Now listening on {}", server.url),
        None => panic!("[server] section must be defined in config!"),
    };

    let interface = Mutex::new(DatabaseInterface::from(
        Database::open(&config.database()).unwrap(),
        WebClient::new(config.timeout),
    ));
    let server = config.server.unwrap();

    rouille::start_server(&server.url, move |request| {
        rouille::log(&request, io::stdout(), || {
            if request.method() == "OPTIONS" {
                rouille::Response::empty_204()
                    .with_additional_header("Access-Control-Allow-Origin", "*")
                    .with_additional_header(
                        "Access-Control-Allow-Methods",
                        "POST, GET, DELETE, OPTIONS",
                    )
                    .with_additional_header(
                        "Access-Control-Allow-Headers",
                        "content-type, x-username, x-password",
                    )
                    .with_additional_header("Access-Control-Max-Age", "86400")
            } else if request.header("X-Username").is_none()
                || request.header("X-Password").is_none()
            {
                rouille::Response::text("X-Username and X-Password headers required")
                    .with_status_code(401)
            } else {
                let username = request.header("X-Username").unwrap();
                let password = request.header("X-Password").unwrap();
                if username != &server.username || password != &server.password {
                    rouille::Response::text("Username or password incorrect").with_status_code(401)
                } else {
                    router!(request,
                        (POST) (/add) => {
                            add(&mut interface.lock().unwrap(), request)
                        },
                        (GET) (/search) => {
                            search(&mut interface.lock().unwrap(), request)
                        },
                        (DELETE) (/search) => {
                            delete(&mut interface.lock().unwrap(), request)
                        },
                        (GET) (/tags) => {
                            tags(&mut interface.lock().unwrap(), request)
                        },
                        _ => rouille::Response::empty_404()
                    )
                    .with_additional_header("Access-Control-Allow-Origin", "*")
                }
            }
        })
    });
}
