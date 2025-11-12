use std::{ ffi::OsString, fmt::Error, path::PathBuf};

use calamine::{Reader, SheetVisible, open_workbook_auto};
use datetime::LocalDateTime;
use tiktoken_rs::o200k_base;

#[derive(Debug)]
pub struct Document {
  file_path: PathBuf,
  file_extension: OsString,
  word_count: u32,
  created_on: LocalDateTime,
  last_modified_on: LocalDateTime,
  token_usage: u32
}

impl Document {
  pub fn new(file_path: PathBuf) -> Self {
    let file_extension = file_path.extension().unwrap().to_owned();
    Self {
      file_path,
      file_extension,
      word_count: 0,
      created_on: LocalDateTime::at_ms(0, 0),
      last_modified_on: LocalDateTime::at_ms(0, 0),
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


  fn load_worksheet(&mut self) {
    let mut workbook = open_workbook_auto(&self.file_path).expect("Cannot open file");

    let sheet_names: Vec<String> = workbook.sheets_metadata()
      .iter()
      .filter(|sheet| sheet.visible == SheetVisible::Visible)
      .map(|sheet| sheet.name.clone())
      .collect();
    
    let mut all_words: Vec<String> = Vec::new();
    for sheet_name in sheet_names {
      let Ok(range) = workbook.worksheet_range(&sheet_name) else { continue };
      
      let sheet_words: Vec<String> = range.rows()
        .flat_map(|row| row.iter())
        .filter_map(|cell| match cell {
          calamine::Data::String(text) => {
        let (words, count) = self.count_word(text);
        self.word_count += count;
        Some(words)
          },
          _ => None
        })
        .flatten()
        .collect();
      
      all_words.extend(sheet_words);
    }

    if let Ok(metadata) = std::fs::metadata(&self.file_path) {
      if let Ok(created) = metadata.created() {
        let created_on = created
          .duration_since(std::time::UNIX_EPOCH)
          .unwrap()
          .as_secs() as i64;
        self.created_on = LocalDateTime::at_ms(created_on, 0);
      }
      if let Ok(modified) = metadata.modified() {
        let last_modified_on = modified
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
          self.last_modified_on = LocalDateTime::at_ms(last_modified_on, 0);
      }
    }

    let bpe = o200k_base().unwrap();

    let mut tokens: Vec<u32> = Vec::new();
    for word in all_words {
      tokens.extend(bpe.encode_with_special_tokens(&word));
    }

    self.token_usage = tokens.len() as u32;
  }

  pub fn load(&mut self) -> Result<(), Error> {
    match self.file_extension.to_str() {
      Some("xlsx") => {
        self.load_worksheet();
        Ok(())
      },
      _ => Err(Error)
    }
  }
}