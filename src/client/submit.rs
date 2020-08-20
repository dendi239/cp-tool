use crate::errors;

use async_trait::async_trait;
use errors::{Error, Result};
use std::path::PathBuf;
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
pub trait SubmitClient {
    // TODO: Replace () with any sutable data associated with submission
    async fn submit(&self, submission: &Submission) -> Result<()>;
}
