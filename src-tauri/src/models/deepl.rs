use std::{fs::File, io::BufReader, path::PathBuf};

use reqwest::{Error, Response};
use serde_json::{json, Value};
use tauri::http::HeaderMap;
use calamine::{Data, DataType, Reader, Xlsx, open_workbook};

use super::model::APIClient;
use std::io::BufRead;

use crate::structs::{Dictionary::*, Glossary::GlossaryType, Language::*};

pub trait Glossary {
  fn dictionaries_from_excel(&self, excel_file_path: PathBuf) -> Vec<Dictionary>;
  fn dictionaries_from_json(&self, json: PathBuf) -> Vec<Dictionary>;
  fn dictionaries_from_text_file(&self, file: PathBuf) -> Vec<Dictionary>;
  async fn create_glossary(&self, file: PathBuf) -> Value;
  async fn get_glossaries(&self) -> Value;
  async fn delete_glossary(&self, glossary_id: String) -> Value;
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

  pub async fn get_target_languages(&self) -> Result<Vec<Language>, Error> {
    let resp: Response = self.client.get("https://api-free.deepl.com/v2/languages?type=target").await;
    let resp = resp.error_for_status()?;

    let result: Value = resp.json().await.unwrap();

    let mut languages: Vec<Language> = Vec::new();

    for language_object in result.as_array().unwrap_or(&vec![]) {
      let language: Language = Language::new(
        language_object["name"].as_str().unwrap_or("").to_string(),
        language_object["language"].as_str().unwrap_or("").to_string()
      );

      languages.push(language);
    }

    Ok(languages)
  }

  pub async fn get_source_languages(&self) -> Result<Vec<Language>, Error> {
    let resp: Response = self.client.get("https://api-free.deepl.com/v2/languages?type=source").await;
    let resp = resp.error_for_status()?;

    let result: Value = resp.json().await.unwrap();

    let mut languages: Vec<Language> = Vec::new();

    for language_object in result.as_array().unwrap_or(&vec![]) {
      let language: Language = Language::new(
        language_object["name"].as_str().unwrap_or("").to_string(),
        language_object["language"].as_str().unwrap_or("").to_string()
      );

      languages.push(language);
    }

    Ok(languages)
  }
}

impl Glossary for Deepl {
  fn dictionaries_from_excel(&self, excel_file_path: PathBuf) -> Vec<Dictionary> {
      // I believe this works, couldn't test properly because I keep getting quota exceeded message
      // Despite not using the API

      let mut workbook: Xlsx<_> = open_workbook(&excel_file_path).expect("Cannot open file");

      let mut dictionaries: Vec<Dictionary> = Vec::new();

      for sheet_name in workbook.sheet_names() {
        let mut source_lang: LanguageCode = String::new();
        let mut target_lang: LanguageCode = String::new();

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

        let dict: Dictionary = Dictionary::new(
          source_lang,
          target_lang,
          entries,
          "csv".to_string()
        );

        dictionaries.push(dict);
      }

      dictionaries
    }
  
  async fn create_glossary(&self, file: PathBuf) -> Value {
    let file_extension = file.extension().unwrap_or_else(|| panic!("Could not load file"));
    let filename = file.file_stem()
      .unwrap_or_else(|| panic!("Could not load file"))
      .to_str()
      .unwrap();

    let dictionaries = match file_extension.to_str() {
      Some("xlsx") => {
        self.dictionaries_from_excel(file.clone())
      },
      Some("txt") | Some("csv") => {
        self.dictionaries_from_text_file(file.clone())
      },
      Some("json") => {
        self.dictionaries_from_json(file.clone())
      }
      _ => panic!("File not supported")
    };

    let glossary = GlossaryType::new(filename, dictionaries);

    let resp = self.client.post_json("https://api-free.deepl.com/v3/glossaries", &json!(glossary)).await;
    
    let resp: Value = resp.json().await.unwrap();
    
    resp
  }
  
  fn dictionaries_from_json(&self, json: PathBuf) -> Vec<Dictionary> {
    let file = File::open(json).unwrap_or_else(|_| panic!("Could not open json file"));
    let reader = BufReader::new(file);

    let object: Value = serde_json::from_reader(reader).unwrap();

    let mut dictionaries: Vec<Dictionary> = Vec::new();

    if object.is_array() {
      for language_object in object.as_array().unwrap_or(&vec![]) {
        let source_lang = language_object["source_lang"].to_string();
        let target_lang = language_object["target_lang"].to_string();

        let mut entries:String = String::new();

        for (key, value) in language_object.as_object().unwrap().iter() {
          if key != "source_lang" && key != "target_lang" {
            entries.push_str(&format!("{},{}\n", key, value.to_string()));
          }
        }

        let dict = Dictionary::new(
          source_lang,
          target_lang,
          entries,
          "csv".to_string()
        );

        dictionaries.push(dict);
      }
    } else if object.is_object() {
      let source_lang = object["source_lang"].to_string();
      let target_lang = object["target_lang"].to_string();

      let mut entries:String = String::new();

      for (key, value) in object.as_object().unwrap().iter() {
        if key != "source_lang" && key != "target_lang" {
          entries.push_str(&format!("{},{}\n", key, value.to_string()));
        }
      }

      let dict = Dictionary::new(
        source_lang,
        target_lang,
        entries,
        "csv".to_string()
      );

      dictionaries.push(dict);
  }

    dictionaries
  }

  fn dictionaries_from_text_file(&self, file: PathBuf) -> Vec<Dictionary> {
    let file = File::open(file).expect("Could not open file");
    let reader = BufReader::new(file);
    
    let mut source_lang = String::new();
    let mut target_lang = String::new();
    let mut entries = String::new();

    for (line_index, line) in reader.lines().enumerate() {
      let line = line.expect("Could not read line");
      let splitted_line: Vec<&str> = line.split(",").collect();

      if line_index == 0 {
        source_lang = splitted_line[0].to_string();
        target_lang = splitted_line[1].to_string();
        continue;
      }

      entries.push_str(&format!("{},{}\n", splitted_line[0], splitted_line[1]));
    }

    let dictionary = Dictionary::new(
      source_lang,
      target_lang,
      entries,
      "csv".to_string()
    );

    // I don't see a case where the user will try to pass more than one dict
    // from a single txt or csv file
    let dictionaries:Vec<Dictionary> = vec![dictionary];

    dictionaries
  }

  async fn get_glossaries(&self) -> Value {
    let resp = self.client.get("https://api-free.deepl.com/v3/glossaries").await;
    let resp: Value = resp.json().await.unwrap();
      
    resp
  }

  async fn delete_glossary(&self, glossary_id: String) -> Value {
    let resp = self.client.delete(&format!("https://api-free.deepl.com/v3/glossaries/{}", glossary_id).as_str()).await;
    let resp = resp.json().await.unwrap();

    resp
  }
}