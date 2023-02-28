use clap::{Parser, Subcommand};
use log;
use plogger;

#[derive(Parser)]
#[clap(about, arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    /// Enable debug mode
    #[clap(short, long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the start command
    Start(start::StartCommand),
    /// Shorthand to run the start command
    S(start::StartCommand),
    /// Initialize the d.toml configuration file
    Init(init::InitCommand),
}

mod configuration;
mod init;
mod start;

fn main() {
    let cli = Cli::parse();

    plogger::init(cli.debug);

    log::info!("Welcome to d - the development helper");

    match cli.command {
        Some(Commands::Start(command)) => {
            let config =
                configuration::Configuration::from_path().expect("Could not load configuration");
            start::command(&command, &config);
        }
        Some(Commands::S(command)) => {
            let config =
                configuration::Configuration::from_path().expect("Could not load configuration");
            start::command(&command, &config);
        }
        Some(Commands::Init(command)) => {
            init::command(&command);
        }
        None => {} // Handled by Clap
    }
}
