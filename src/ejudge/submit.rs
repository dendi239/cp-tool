use super::client::Client;

use crate::client::{Submission, SubmitClient};
use crate::errors::Result;

use async_trait::async_trait;

#[async_trait]
impl SubmitClient for Client {
    async fn submit(&self, submission: &Submission) -> Result<()> {
        let pid = submission.get_problem_id()?;

        let submit_data: [(&str, &str); 5] = [
            ("prob_id", &pid),
            ("lang_id", &submission.lang_id),
            ("file", &submission.get_source()?),
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
