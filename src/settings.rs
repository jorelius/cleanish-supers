use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

use crate::entities::supers::Super;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub supers: Vec<Super>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("config/{}", run_mode))
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/local").required(false))
            // Add in settings from the environment (with a prefix of SUPERS)
            // Eg.. `SUPERS_DEBUG=1` would set the `debug` key
            .add_source(Environment::with_prefix("SUPERS"))
            .build()?;

        s.try_deserialize()
    }
}