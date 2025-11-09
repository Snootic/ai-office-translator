use reqwest::{Error, Response};
use serde_json::{json, Value};
use tauri::http::HeaderMap;
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};

use super::model::APIClient;

pub trait Glossary {
  async fn create_glossary_from_excel(&self, excel_file_path: String) -> Value;
  fn create_glossary(&self) -> Value;
  fn load_json(&self, json: Value) -> Value;
  fn get_glossaries(&self) -> Vec<Value>;
}

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

impl Glossary for Deepl {
  async fn create_glossary_from_excel(&self, excel_file_path: String) -> Value {
      // I believe this works, couldn't test properly because I keep getting quota exceeded message
      // Despite not using the API

      let mut workbook: Xlsx<_> = open_workbook(&excel_file_path).expect("Cannot open file");

      let mut filename = excel_file_path.split(".xls").next().unwrap_or("");
      filename = filename.split("/").last().unwrap_or("");

      let mut dictionaries: Vec<Value> = [].to_vec();

      for sheet_name in workbook.sheet_names() {
        let mut source_lang: String = String::new();
        let mut target_lang: String = String::new();

        let mut entries: String = String::new();

        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
          for (row_index, row) in range.rows().enumerate() {
            let non_empty_cells: Vec<&Data> = row.iter().filter(|cell| !cell.is_empty()).collect();
            
            if non_empty_cells.len() != 2 {
              continue;
            }

            if row_index == 0 {
              source_lang = row[0].as_string().unwrap_or_default();
              target_lang = row[1].as_string().unwrap_or_default();
              continue;
            }

            let col1 = row[0].as_string().unwrap_or_default();
            let col2 = row[1].as_string().unwrap_or_default();

            entries.push_str(&format!("{},{}\n", col1, col2));
          }
        }

        let dict = json!({
          "source_lang": source_lang,
          "target_lang": target_lang,
          "entries": entries,
          "entries_format": "csv"
        });

        dictionaries.push(dict);
      }

      let body = json!({
        "name": filename,
        "dictionaries": dictionaries
      });

      let resp = self.client.post_json("https://api-free.deepl.com/v3/glossaries", &body).await;
      
      let resp: Value = resp.json().await.unwrap();
      
      resp
    }
  
  fn create_glossary(&self) -> Value {
        todo!()
    }
  
  fn load_json(&self, json: Value) -> Value {
        todo!()
    }
  
  fn get_glossaries(&self) -> Vec<Value> {
        todo!()
    }
}