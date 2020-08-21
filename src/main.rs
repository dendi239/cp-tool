mod client;
mod ejudge;
mod errors;
mod login;

use client::{Config, ConfigClient, SubmitClient};
use errors::Result;
use login::ContestInfo;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "login", about = "logs you in specified contest.")]
    Login(login::ContestInfo),

    #[structopt(name = "submit", about = "Submits FILE to given PROBLEM.")]
    Submit(client::Submission),
}

// TODO: Return client just built or something.
async fn save_login(contest_info: &ContestInfo, into_path: &PathBuf) -> Result<()> {
    match contest_info {
        ContestInfo::Ejudge(info) => ejudge::read_login(info).await?.save_config(&into_path),
        // TODO: add additional judge systems here
    }
}

fn from_concrete_config<T: client::Client>(config: T::Config) -> Result<T> {
    T::from_config(config)
}

fn from_config(config: Config) -> Result<Box<dyn SubmitClient>> {
    Ok(Box::new(match config {
        Config::Ejudge(config) => from_concrete_config::<ejudge::Client>(config),
        // TODO: add additional judge systems here
    }?))
}

/// Finds config in enviroment: scans all parent directories until config'd be found.
fn from_env(current_direcrtory: &PathBuf) -> Result<Box<dyn SubmitClient>> {
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
    let current_path = std::env::current_dir()?;
    match Command::from_args() {
        Command::Login(info) => save_login(&info, &current_path).await,
        Command::Submit(submission) => from_env(&current_path)?.submit(&submission).await,
    }
}
