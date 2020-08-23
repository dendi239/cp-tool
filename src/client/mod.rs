mod config;
mod submit;

pub use config::{AsClientConfig, ConfigClient};
pub use submit::{Submission, SubmitClient};

use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait Client<Config: Serialize>: SubmitClient + ConfigClient<Config> {}
