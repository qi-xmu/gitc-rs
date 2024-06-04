use std::path::PathBuf;

use anyhow::Result;
use dirs;
use json::object;

const DEFAULT_CONFIG_NAME: &str = ".gitc";

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub bot_id: String,
    pub token: String,
    pub confirm: bool,
}

impl Config {
    pub fn from_config(path: &PathBuf) -> Self {
        // 读取文件
        let content = std::fs::read_to_string(path).unwrap();
        let config_json = json::parse(&content).unwrap();

        Config {
            bot_id: config_json["bot_id"].to_string(),
            token: config_json["token"].to_string(),
            confirm: config_json["confirm"].as_bool().unwrap_or(true),
        }
    }

    pub fn save_config(&self, path: &PathBuf) -> Result<()> {
        let config_json = object! {
            "bot_id": self.bot_id.to_owned(),
            "token": self.token.to_owned(),
            "confirm": self.confirm
        };

        std::fs::write(path, config_json.dump())?;
        Ok(())
    }
}

pub fn read_config() -> Option<Config> {
    let home_dir = dirs::home_dir().unwrap();
    let path = home_dir.join(DEFAULT_CONFIG_NAME);

    if path.exists() {
        let config = Config::from_config(&path);

        if config.bot_id.is_empty() || config.token.is_empty() {
            None
        } else {
            Some(config)
        }
    } else {
        let config = Config::default();
        config.save_config(&path).unwrap();
        None
    }
}
