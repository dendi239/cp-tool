use crate::errors;

use async_trait::async_trait;
use errors::Result;
use serde::Serialize;
use std::path::PathBuf;

pub trait AsClientConfig<Config> {
    fn as_client_config(self) -> Config;
}

#[async_trait]
pub trait ConfigClient<Config: Serialize>: Sized {
    type Config: serde::Serialize + AsClientConfig<Config>;

    /// Builds client with given config.
    fn from_config(config: Self::Config) -> Result<Self>;

    /// Gets config from client in order capable to recover state using `from_config`.
    fn get_config(&self) -> Self::Config;

    // TODO: replace () with any suitable data
    fn save_config(&self, path: &PathBuf) -> Result<()> {
        let file_path = path.clone().join(".cp-tool.config");
        let client_config = self.get_config().as_client_config();
        let config_string = serde_json::to_string_pretty(&client_config)?;

        println!("filepath to save is: {:?}", file_path);
        println!("Stored config is: {}", config_string);

        std::fs::write(file_path, config_string)?;
        Ok(())
    }
}
