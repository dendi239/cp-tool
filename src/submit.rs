use crate::client;
use crate::errors;

use client::EjudgeClient;
use errors::{Error, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Submission {
    #[structopt(short, long)]
    problem_id: Option<String>,

    #[structopt(short, long, default_value = "3")]
    lang_id: String,

    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

use std::string::String;

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

fn get_problem_id(submission: &Submission) -> Result<String> {
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

impl EjudgeClient {
    pub async fn submit(&self, submission: &Submission) -> Result<()> {
        let pid = &get_problem_id(submission)?;

        let submit_data: [(&str, &str); 5] = [
            ("prob_id", pid),
            ("lang_id", &submission.lang_id),
            ("file", &std::fs::read_to_string(&submission.file)?),
            ("SID", &self.session_id),
            ("action_40", "Send!"),
        ];

        let submit_response = self
            .client
            .post(self.base_url.clone())
            .form(&submit_data)
            .send()
            .await?;

        // TODO: Check if session_id is valid.
        //       There're must be sid as query parameter in response.
        println!("Submit response url: {}", submit_response.url());
        open::that(submit_response.url().clone().into_string())?;

        Ok(())
    }
}
