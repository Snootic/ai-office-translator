use reqwest::{Error, Response};
use serde_json::Value;

use crate::structs::api_client::APIClient;
use crate::structs::chatgpt::ChatGPT;


impl ChatGPT {
  pub fn new(api_key: String) -> Self {
    Self {
      client: APIClient::new(api_key, None),
    }
  }

  pub async fn list_models(&self) -> Result<Vec<String>, Error> {
    let resp: Response = self.client.get("https://api.openai.com/v1/models").await;
    let resp = resp.error_for_status()?;
    let json: Value = resp.json().await?;

    let allowed = [
      "gpt-3.5-turbo",
      "gpt-4",
      "gpt-4-turbo",
      "gpt-4o",
      "gpt-4o-mini",
      "gpt-4.1",
      "gpt-4.1-mini",
      "gpt-5",
      "gpt-5-mini",
      "gpt-5-pro",
    ];

    let all_models: &Vec<Value> = json.get("data").and_then(| value | value.as_array()).unwrap();

    let mut filtered_models: Vec<String> = Vec::new();

    for model in all_models {
      let id = model.get("id").unwrap().as_str().unwrap().to_string();
      if allowed.iter().any(|ex| id == *ex) {
        filtered_models.push(id);
      }
    }

    Ok(filtered_models)
  }

  pub async fn get_monthly_billing(&self) -> Result<Value, Error> {
    let resp: Response = self.client.get("https://api.openai.com/v1/organization/costs").await;
    let resp = resp.error_for_status()?;
    
    let json: Value = resp.json().await?;

    Ok(json)
  }
}