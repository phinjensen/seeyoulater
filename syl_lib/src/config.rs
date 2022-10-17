use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::Deserialize;
use toml;

// TODO: Make these fields private (and make the Config object create the interfaces, maybe?)
#[derive(Deserialize, Debug)]
pub struct Config {
    pub db_file: Option<String>,
    pub server: Option<Server>,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn open(path: Option<String>) -> Self {
        let path = if let Some(path_string) = path {
            PathBuf::from(path_string)
        } else {
            if let Some(dirs) = ProjectDirs::from("com", "phinjensen", "seeyoulater") {
                let dir = dirs.config_dir();
                fs::create_dir_all(dir).unwrap();
                dir.join("config.toml")
            } else {
                panic!("Error finding config directory.");
            }
        };

        if let Ok(config_string) = fs::read_to_string(path) {
            toml::from_str(&config_string).unwrap()
        } else {
            Config {
                server: None,
                db_file: None,
            }
        }
    }

    //TODO: This fails unless the directory is already created. Fix that
    pub fn database(&self) -> String {
        if let Some(dirs) = ProjectDirs::from("com", "phinjensen", "seeyoulater") {
            let dir = dirs.data_dir();
            fs::create_dir_all(dir).unwrap();
            dir.join("seeyoulater.db").to_str().unwrap().to_string()
        } else {
            panic!("Error opening data directory!");
        }
    }
}
