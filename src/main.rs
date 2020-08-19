mod client;
mod ejudge;
mod errors;
mod login;

use client::Client;
use errors::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "login", about = "logs you in specified contest.")]
    Login(login::ContestInfo),

    #[structopt(name = "submit", about = "Submits FILE to given PROBLEM.")]
    Submit(client::Submission),
}

#[tokio::main]
async fn main() -> Result<()> {
    match Command::from_args() {
        Command::Login(contest_info) => {
            ejudge::read_login(&contest_info)
                .await?
                .save_config(std::env::current_dir()?)?;
        }
        Command::Submit(submission) => {
            ejudge::EjudgeClient::from_env()?
                .submit(&submission)
                .await?
        }
    };

    Ok(())
}
