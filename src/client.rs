use crate::errors;

use async_trait::async_trait;
use errors::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::string::String;
use structopt::StructOpt;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Config {
    Ejudge {
        contest_url: String,
        session_id: String,
    },
}

#[derive(Debug, StructOpt)]
pub struct Submission {
    #[structopt(short, long)]
    pub problem_id: Option<String>,

    #[structopt(short, long, default_value = "3")]
    pub lang_id: String,

    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
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
    /// Builds client with given config.
    fn from_config(config: Config) -> Result<Self>;

    // TODO: replace () with any suitable data
    fn save_config(&self, path: std::path::PathBuf) -> Result<()>;

    // TODO: Replace () with any sutable data associated with submission
    async fn submit(&self, submission: &Submission) -> Result<()>;

    /// Finds config in enviroment: scans all parent directories until config'd be found.
    fn from_env() -> Result<Self> {
        let current_direcrtory = std::env::current_dir()?;
        let curr_dir = current_direcrtory.as_path();
        std::iter::successors(Some(curr_dir), |&x| x.parent())
            .filter_map(|path| std::fs::read(path.join(".cp-tool.config")).ok())
            .filter_map(|file| serde_json::from_slice(&file).ok())
            .filter_map(|config| Self::from_config(config).ok())
            .next()
            .ok_or(errors::Error::MissingConfig)
    }
}
