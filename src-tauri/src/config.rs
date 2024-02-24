use crate::driven::repository::sqlite::{SQLITE_FILE, SQLITE_LOCAL_PATH};

pub struct Config {
    pub app_config_dir: String,
    pub sqlite: SqtliteConfig,
}

pub struct SqtliteConfig {
    pub db_path: String,
}

impl SqtliteConfig {
    pub fn from(app_config_dir: String) -> Self {
        let db_path = format!("{}/{}/{}", app_config_dir, SQLITE_LOCAL_PATH, SQLITE_FILE);
        Self { db_path }
    }
}

impl Config {
    pub fn from(app_config_dir: String) -> Self {
        let sqlite = SqtliteConfig::from(app_config_dir.clone());

        Self {
            app_config_dir,
            sqlite,
        }
    }
}
