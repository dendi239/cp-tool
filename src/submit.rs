use crate::client;
use crate::errors;

use client::EjudgeClient;
use errors::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Submission {
    #[structopt(short, long)]
    problem_id: String,

    #[structopt(short, long, default_value = "3")]
    lang_id: String,

    #[structopt(long, parse(from_os_str))]
    file: PathBuf,
}

impl EjudgeClient {
    pub async fn submit(&self, submission: &Submission) -> Result<()> {
        let submit_data: [(&str, &str); 5] = [
            ("prob_id", &submission.problem_id),
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

        println!("Submit response status: {}", submit_response.status());
        Ok(())
    }
}
