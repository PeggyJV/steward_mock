//! Steward Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};

/// Steward Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StewardConfig {
    /// An example configuration section
    pub server: ServerSection,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on StewardConfig instead.
impl Default for StewardConfig {
    fn default() -> Self {
        Self {
            server: ServerSection::default(),
        }
    }
}

/// Example configuration section.
///
/// Delete this and replace it with your actual configuration structs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ServerSection {
    /// Example configuration value
    pub port: u16,
}

impl Default for ServerSection {
    fn default() -> Self {
        Self {
            port: 0,
        }
    }
}
