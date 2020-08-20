use crate::ejudge;
use crate::errors;

use async_trait::async_trait;
use errors::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::string::String;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Submission {
    #[structopt(short, long)]
    pub problem_id: Option<String>,

    #[structopt(short, long, default_value = "3")]
    pub lang_id: String,

    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientConfig {
    Ejudge(<ejudge::EjudgeClient as Client>::Config),
}

pub trait AsClientConfig {
    fn as_client_config(self) -> ClientConfig;
}

impl AsClientConfig for <ejudge::EjudgeClient as Client>::Config {
    fn as_client_config(self) -> ClientConfig {
        ClientConfig::Ejudge(self)
    }
}

fn get_problem_id_from_filename(name: &std::ffi::OsStr) -> Option<String> {
    if name.len() == 1 {
        let order = match name.to_str()?.chars().next().unwrap() {
            x @ 'A'..='Z' => x as u32 - 'A' as u32 + 1,
            x @ 'a'..='z' => x as u32 - 'a' as u32 + 1,
            _ => {
                return None;
            }
        };

        Some(format!("{}", order))
    } else {
        None
    }
}

pub fn get_problem_id(submission: &Submission) -> Result<String> {
    if let Some(pid) = &submission.problem_id {
        Ok(pid.clone())
    } else {
        [&std::env::current_dir()?, &submission.file]
            .iter()
            .filter_map(|path| path.file_stem())
            .filter_map(|name| get_problem_id_from_filename(&name))
            .next()
            .ok_or(Error::MissingProblemId)
    }
}

#[async_trait]
pub trait Client: Sized {
    type Config: serde::Serialize + AsClientConfig;

    // TODO: Replace () with any sutable data associated with submission
    async fn submit(&self, submission: &Submission) -> Result<()>;

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
