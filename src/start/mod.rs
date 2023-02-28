use clap::Parser;
use std::error::Error;
use std::process::{Command, Stdio};

use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::configuration::Configuration;

#[derive(Parser)]
pub struct StartCommand {
    /// Enable debug mode
    #[clap(short, long, global = true)]
    debug: bool,
}

pub fn command(
    command: &StartCommand,
    configuration: &Configuration,
) -> Result<String, Box<dyn std::error::Error>> {
    log::debug!("Running Start command: {:?}", configuration);

    if configuration.start.is_valid() == false {
        log::error!("Start command is not configured, please check your d.toml file and update it accordingly.");
        return Ok(String::from("Err"));
    }

    let mut child = Command::new(&configuration.start.command)
        .args(configuration.start.args())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Couldn't run program");

    let _status = child.wait();

    log::debug!("Command execution finished, exiting");

    Ok(String::from("Ok"))
}
