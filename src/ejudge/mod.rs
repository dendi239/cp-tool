mod client;
mod config;
mod login;
mod submit;

use crate::client::Client as ClientTrait;
use crate::Config as CrateConfig;

pub use client::Client;
pub use config::Config;
pub use login::{read_login, UrlContestIDInfo};

impl ClientTrait<CrateConfig> for Client {}
