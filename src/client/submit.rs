use crate::errors;

use async_trait::async_trait;
use errors::{Error, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Submission {
    #[structopt(short, long)]
    problem_id: Option<String>,

    #[structopt(short, long, default_value = "3")]
    pub lang_id: String,

    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

#[async_trait]
pub trait SubmitClient {
    // TODO: Replace () with any sutable data associated with submission
    async fn submit(&self, submission: &Submission) -> Result<()>;
}

impl Submission {
    pub fn get_source(&self) -> Result<String> {
        Ok(std::fs::read_to_string(&self.file)?)
    }

    pub fn get_problem_id(&self) -> Result<String> {
        if let Some(pid) = &self.problem_id {
            Ok(pid.clone())
        } else {
            [&std::env::current_dir()?, &self.file]
                .iter()
                .filter_map(|path| path.file_stem())
                .filter_map(|name| get_problem_id_from_filename(&name))
                .next()
                .ok_or(Error::MissingProblemId)
        }
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
