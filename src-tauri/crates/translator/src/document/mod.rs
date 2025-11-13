pub mod doc;
pub mod ppt;
pub mod xls;

use doc::Doc;
use xls::Xls;

use std::{ ffi::OsString, path::PathBuf};

use serde::{Deserialize, Serialize};
use tiktoken_rs::o200k_base;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
  #[serde(skip)]
  file_path: PathBuf,
  #[serde(skip)]
  file_extension: OsString,
  word_count: u32,
  created_on: i64,
  last_modified_on: i64,
  token_usage: u32
}

impl Document {
  pub fn new(file_path: PathBuf) -> Self {
    let file_extension = file_path.extension().unwrap().to_owned();
    Self {
      file_path,
      file_extension,
      word_count: 0,
      created_on: 0,
      last_modified_on: 0,
      token_usage: 0
    }
  }

  fn is_cjk(&self, c: char) -> bool {
    matches!(c,
      '\u{4E00}'..='\u{9FFF}' | // CJK Unified Ideographs
      '\u{3400}'..='\u{4DBF}' | // CJK Unified Ideographs Extension A
      '\u{3040}'..='\u{309F}' | // Hiragana
      '\u{30A0}'..='\u{30FF}' | // Katakana
      '\u{AC00}'..='\u{D7AF}'   // Hangul Syllables
    )
  }

  fn count_word(&self, text: &str) -> (Vec<String>, u32) {
    let mut words = Vec::new();
    let mut count = 0;
    let mut current_word = String::new();
    let mut in_word = false;
    
    for c in text.chars() {
      if c.is_whitespace() {
        if in_word && !current_word.is_empty() {
          words.push(current_word.clone());
          current_word.clear();
        }
        in_word = false;
      } else if self.is_cjk(c) {
        if in_word && !current_word.is_empty() {
          words.push(current_word.clone());
          current_word.clear();
        }
        words.push(c.to_string());
        count += 1;
        in_word = false;
      } else if c.is_alphanumeric() {
        if !in_word {
          count += 1;
          in_word = true;
        }
        current_word.push(c);
      } else {
        if in_word && !current_word.is_empty() {
          words.push(current_word.clone());
          current_word.clear();
        }
        in_word = false;
      }
    }
    
    if !current_word.is_empty() {
      words.push(current_word);
    }
    
    (words, count)
  }

  fn get_dates_metadata(&mut self) {
    if let Ok(metadata) = std::fs::metadata(&self.file_path) {
      if let Ok(created) = metadata.created() {
        self.created_on = created
          .duration_since(std::time::UNIX_EPOCH)
          .unwrap()
          .as_secs() as i64;
      }
      if let Ok(modified) = metadata.modified() {
        self.last_modified_on = modified
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
      }
    }
  }

  fn calculate_tokens(&mut self, words: Vec<String>) {
    let bpe = o200k_base().unwrap();
    let mut tokens: Vec<u32> = Vec::new();
    for word in words {
      tokens.extend(bpe.encode_with_special_tokens(&word));
    }

    self.token_usage = tokens.len() as u32;
  }

  pub fn load(&mut self) -> Result<(), String> {
    let ext = self.file_extension.to_str().unwrap_or("").to_lowercase();
    
    match ext.as_str() {
      "doc" | "docx" => {
        Doc::load(self);
        Ok(())
      }
      "xls" | "xlsx" => {
        Xls::load(self);
        Ok(())
      }
      _ => Err(format!("Unsupported file extension: {}", ext))
    }
  }

  
}