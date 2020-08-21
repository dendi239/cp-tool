pub struct Client {
    pub session_id: String,
    pub base_url: url::Url,
    pub client: reqwest::Client,
}
