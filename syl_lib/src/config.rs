use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use serde::Deserialize;
use toml;

pub enum ConfigPath {
    Custom(String),
    ServerDefault,
    ClientDefault,
}

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
    pub fn open(path: ConfigPath) -> Self {
        let path = if let ConfigPath::Custom(path_string) = path {
            PathBuf::from(path_string)
        } else {
            if let Some(dirs) = ProjectDirs::from("com", "phinjensen", "seeyoulater") {
                let dir = dirs.config_dir();
                fs::create_dir_all(dir).unwrap();
                match path {
                    ConfigPath::ServerDefault => dir.join("config-server.toml"),
                    ConfigPath::ClientDefault => dir.join("config.toml"),
                    _ => dir.to_path_buf(), // Shouldn't be possible
                }
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

    pub fn database(&self) -> String {
        match &self.db_file {
            Some(path) => path.to_string(),
            None => {
                if let Some(dirs) = ProjectDirs::from("com", "phinjensen", "seeyoulater") {
                    let dir = dirs.data_dir();
                    fs::create_dir_all(dir).unwrap();
                    dir.join("seeyoulater.db").to_str().unwrap().to_string()
                } else {
                    panic!("Error opening data directory!");
                }
            }
        }
    }
}
