mod config;
mod submit;

pub use config::{AsClientConfig, ClientConfig, ConfigClient};
pub use submit::{Submission, SubmitClient};

use async_trait::async_trait;

#[async_trait]
pub trait Client: SubmitClient + ConfigClient {}
