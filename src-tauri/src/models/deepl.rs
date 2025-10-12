use reqwest::{Error, Response};
use serde_json::Value;
use tauri::http::HeaderMap;

use super::model::APIClient;

pub struct Deepl {
  client: APIClient,
}

impl Deepl {
  pub fn new(api_key: String) -> Self {
    Self {
      client: {
        let mut headers = HeaderMap::new();
        let auth_value = format!("DeepL-Auth-Key {}", api_key);
        headers.insert("Authorization", auth_value.parse().unwrap());
        APIClient::new(api_key.clone(), Some(headers))
      }
    }
  }

  pub async fn check_usage(&self) -> Result<Value, Error> {
    let resp: Response = self.client.get("https://api-free.deepl.com/v2/usage").await;
    let resp = resp.error_for_status()?;
    let usage = resp.json().await.unwrap();

    Ok(usage)
  }

  pub async fn get_target_languages(&self) -> Result<Value, Error> {
    let resp: Response = self.client.get("https://api-free.deepl.com/v2/languages?type=target").await;
    let resp = resp.error_for_status()?;

    let languages = resp.json().await.unwrap();

    Ok(languages)
  }

  pub async fn get_source_languages(&self) -> Result<Value, Error> {
    let resp: Response = self.client.get("https://api-free.deepl.com/v2/languages?type=source").await;
    let resp = resp.error_for_status()?;

    let languages = resp.json().await.unwrap();

    Ok(languages)
  }
}