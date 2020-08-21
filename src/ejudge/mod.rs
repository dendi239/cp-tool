mod client;
mod config;
mod login;
mod submit;

use crate::client::Client as ClientTrait;

pub use client::Client;
pub use login::{read_login, UrlContestIDInfo};

impl ClientTrait for Client {}
