//! Steward Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod start;

use self::{start::StartCmd};
use crate::config::StewardConfig;
use abscissa_core::{
    config::Override, Command, Configurable, FrameworkError, Clap, Runnable,
};
use std::path::PathBuf;

/// Steward Configuration Filename
pub const CONFIG_FILE: &str = "steward.toml";

/// Steward Subcommands
#[derive(Command, Debug, Clap, Runnable)]
pub enum StewardCmd {
    /// The `start` subcommand
    Start(StartCmd),
}

/// Entry point for the application. It needs to be a struct to allow using subcommands!
#[derive(Command, Debug, Clap)]
#[clap(author, about, version)]
pub struct EntryPoint {
    #[clap(subcommand)]
    cmd: StewardCmd,

    /// Enable verbose logging
    #[clap(short, long)]
    pub verbose: bool,

    /// Use the specified config file
    #[clap(short, long)]
    pub config: Option<String>,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run()
    }
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<StewardConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = self
        .config
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| CONFIG_FILE.into());

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(
        &self,
        config: StewardConfig,
    ) -> Result<StewardConfig, FrameworkError> {
        match &self.cmd {
            _ => Ok(config),
        }
    }
}
