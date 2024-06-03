use dirs;
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG_NAME: &str = ".gitc";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    bot_id: String,
    token: String,
}

impl Config {
    pub fn new(bot_id: String, token: String) -> Self {
        Config { bot_id, token }
    }
}

pub fn read_config() {
    let mut home_dir = dirs::home_dir().unwrap();
    home_dir.push(DEFAULT_CONFIG_NAME);
}
