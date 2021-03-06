mod steamcmd;
mod commands;
mod executable;
mod utils;
mod logger;
mod files;

use clap::{App, load_yaml};
use log::{SetLoggerError, LevelFilter, debug};
use crate::logger::OdinLogger;
use crate::executable::handle_exit_status;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
static LOGGER: OdinLogger = OdinLogger;
static GAME_ID: i64 = 896660;

fn setup_logger(debug: bool) -> Result<(), SetLoggerError> {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let result = log::set_logger(&LOGGER)
        .map(|_| log::set_max_level(level));
    debug!("Debugging set to {}", debug.to_string());
    result
}

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let app = App::from(yaml)
        .version(VERSION);

    let matches = app.get_matches();

    setup_logger(matches.is_present("debug")).unwrap();

    if let Some(ref _match) = matches.subcommand_matches("install") {
        let result = commands::install::invoke(GAME_ID);
        handle_exit_status(result, "Successfully installed Valheim!".to_string())
    };

    if let Some(ref start_matches) = matches.subcommand_matches("start") {
        commands::start::invoke(start_matches);
    };

    if let Some(ref stop_matches) = matches.subcommand_matches("stop") {
        commands::stop::invoke(stop_matches);
    };

}
