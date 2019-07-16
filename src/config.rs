use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

use failure::Fail;
use toml;

/// The context config
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub aws_context: Vec<AwsContext>,
}

#[derive(Serialize, Deserialize)]
pub struct AwsContext {
    pub region: String,
    pub account: String,
    pub role: String,
    pub script: Option<String>,
}

impl Config {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, ConfigError> {
        let s = std::fs::read_to_string(&path)?;
        toml::from_str(&s).map_err(|_| ConfigError::InvalidConfig)
    }
}

#[derive(Fail, Debug)]
pub enum ConfigError {
    #[fail(display = "invalid configuration")]
    InvalidConfig,
}

impl From<std::io::Error> for ConfigError {
    fn from(_err: std::io::Error) -> Self {
        ConfigError::InvalidConfig
    }
}
/// Join a path to the HOME directory. Panics on any error.
pub fn home_with(path: &'static str) -> String {
    Path::new(&env::var("HOME").unwrap())
        .join(path)
        .to_str()
        .unwrap()
        .to_owned()
}
