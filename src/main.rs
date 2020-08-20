mod client;
mod ejudge;
mod errors;
mod login;

use client::{ClientConfig, ConfigClient, SubmitClient};
use errors::Result;
use login::ContestInfo;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "login", about = "logs you in specified contest.")]
    Login(login::ContestInfo),

    #[structopt(name = "submit", about = "Submits FILE to given PROBLEM.")]
    Submit(client::Submission),
}

// TODO: Update all necessary code to support any judge system, not only Ejudge one
type AnyClient = ejudge::EjudgeClient;

fn from_config(config: ClientConfig) -> Result<AnyClient> {
    match config {
        ClientConfig::Ejudge(config) => ejudge::EjudgeClient::from_config(config),
        // TODO: add additional judge systems here
    }
}

async fn read_login(contest_info: &login::ContestInfo) -> Result<ejudge::EjudgeClient> {
    match contest_info {
        ContestInfo::Ejudge(info) => ejudge::read_login(info).await,
        // TODO: add additional judge systems here
    }
}

/// Finds config in enviroment: scans all parent directories until config'd be found.
fn from_env() -> Result<AnyClient> {
    let current_direcrtory = std::env::current_dir()?;
    let curr_dir = current_direcrtory.as_path();
    std::iter::successors(Some(curr_dir), |&x| x.parent())
        .filter_map(|path| std::fs::read(path.join(".cp-tool.config")).ok())
        .filter_map(|file| {
            let config = serde_json::from_slice(&file).ok()?;
            from_config(config).ok()
        })
        .next()
        .ok_or(errors::Error::MissingConfig)
}

#[tokio::main]
async fn main() -> Result<()> {
    match Command::from_args() {
        Command::Login(contest_info) => read_login(&contest_info)
            .await?
            .save_config(&std::env::current_dir()?)?,
        Command::Submit(submission) => from_env()?.submit(&submission).await?,
    };

    Ok(())
}
