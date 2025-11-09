use reqwest::Client;
use tauri::http::HeaderMap;

#[derive(Debug)]
pub struct APIClient {
  api_key: String,
  client: Client,
  headers: HeaderMap
}

impl APIClient {
  pub fn new(api_key: String, headers: Option<HeaderMap>) -> Self {
    Self {
      api_key,
      client: Client::new(),
      headers: headers.unwrap_or_else(HeaderMap::new),
    }
  }

  pub async fn get(&self, url: &str) -> reqwest::Response {
    self.client.get(url)
      .bearer_auth(&self.api_key)
      .headers(self.headers.clone())
      .send()
      .await
      .unwrap()
  }

  pub async fn post_json<T: serde::Serialize>(&self, url: &str, body: &T) -> reqwest::Response {
    self.client
    .post(url)
    .bearer_auth(&self.api_key)
    .headers(self.headers.clone())
    .json(body)
    .send()
    .await
    .unwrap()
  }
}