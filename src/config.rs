use serde::Deserialize;
use std::fs::read_to_string;
use crate::error::CustomError;
const CONFIG_FILE_PATH: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) stop_processes_containing_names: Vec<String>,
    pub(crate) start_applications_at_file_paths: Vec<String>,
}

impl Config {
    pub(crate) fn new() -> Result<Config, CustomError> {
        let file_content = match read_to_string(CONFIG_FILE_PATH) {
            Ok(content) => content,
            Err(_) => return Err(CustomError::InvalidFileError(
                format!("Config file {CONFIG_FILE_PATH} not found or isn't valid UTF-8"))
            )
        };

        match toml::from_str(&file_content) {
            Ok(config) => Ok(config),
            Err(_) => Err(CustomError::InvalidConfigError(format!("Failed to parse config file {CONFIG_FILE_PATH}")))
        }
    }
}