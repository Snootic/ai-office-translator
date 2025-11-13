use reqwest::Client;
use reqwest::header::HeaderMap;

#[derive(Debug)]
pub struct APIClient {
  pub(crate) api_key: String,
  pub client: Client,
  pub headers: HeaderMap
}
