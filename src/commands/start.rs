//! `start` subcommand - example of how to write a subcommand

use std::process::exit;

use crate::server::server;
use crate::{application::APPLICATION, prelude::*};

use crate::config::StewardConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};

#[derive(Command, Debug, Options)]
pub struct StartCmd {
    #[options(help = "the port steward will listen on")]
    port: Option<u16>,
}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        let config = app_config();

        abscissa_tokio::run(&APPLICATION, async {
            info!("Listening on port {}...", config.server.port);
            server::serve().await;
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            exit(1);
        });
    }
}

impl config::Override<StewardConfig> for StartCmd {
    fn override_config(
        &self,
        mut config: StewardConfig,
    ) -> Result<StewardConfig, FrameworkError> {
        if !self.port.is_none() {
            config.server.port = self.port.unwrap();
        }

        Ok(config)
    }
}
