use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub start: Command,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Command {
    pub command: String,
    pub arguments: String,
}

impl Command {
    pub fn args(&self) -> Vec<&str> {
        self.arguments.split_whitespace().collect()
    }

    pub fn is_valid(&self) -> bool {
        self.command.eq("") == false
    }
}

impl Configuration {
    pub fn from_path() -> Result<Self, Box<dyn std::error::Error>> {
        let toml_string = fs::read_to_string("d.toml")?;

        log::debug!("Loaded configuration file {:?}", toml_string);

        let parsed_configuration: Configuration = toml::from_str(&toml_string)?;

        log::debug!("Parsed configuration: {:?}", parsed_configuration);

        Ok((parsed_configuration))
    }

    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Configuration {
            start: Command {
                command: String::from(""),
                arguments: String::from(""),
            },
        };

        let toml_string = toml::to_string(&config)?;
        let mut config_file = File::create("d.toml")?;
        config_file.write_all(toml_string.as_bytes())?;

        Ok(config)
    }
}
