use directories::ProjectDirs;

pub struct Config;

impl Config {
    pub fn new() -> Self {
        Config {}
    }

    //TODO: This fails unless the directory is already created. Fix that
    pub fn database(&self) -> String {
        if let Some(dirs) = ProjectDirs::from("com", "phinjensen", "seeyoulater") {
            dirs.data_dir()
                .join("seeyoulater.db")
                .to_str()
                .unwrap()
                .to_string()
        } else {
            panic!("Error opening data directory!");
        }
    }
}
