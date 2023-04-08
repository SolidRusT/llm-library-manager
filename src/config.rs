use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub config_file: String,
    pub data_models: PathBuf,
}

impl Config {
    pub fn new(config_file: &str, data_models: &str) -> Self {
        Self {
            config_file: config_file.to_string(),
            data_models: PathBuf::from(data_models),
        }
    }
}
