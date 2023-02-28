use crate::configuration::Configuration;
use clap::Parser;
use dialoguer::Confirm;

#[derive(Parser)]
pub struct InitCommand {
    /// Enable debug mode
    #[clap(short, long, global = true)]
    debug: bool,
}

pub fn command(_command: &InitCommand) -> Result<String, Box<dyn std::error::Error>> {
    log::debug!("Initializing d configuration");

    let existing_configuration_metadata = std::fs::metadata("./d.toml");

    match existing_configuration_metadata {
        Ok(_) => {
            let confirmed = Confirm::new()
            .with_prompt("An existing d.toml was already found, continuing will override its contents, are you sure?")
            .default(false)
            .interact()
            .unwrap();
            if confirmed {
                log::info!("Proceeding");
            } else {
                log::debug!("Aborting");
                return Ok(String::from("Exited"));
            }
        }
        Err(_) => {}
    }

    let config = Configuration::init()?;

    log::info!("Initialized empty configuration in d.toml");
    log::debug!("Initialized configuration: {:?}", config);
    log::info!("Update the command and arguments values of the start section in the configuration and get started with d start.");

    Ok(String::from("Ok"))
}
