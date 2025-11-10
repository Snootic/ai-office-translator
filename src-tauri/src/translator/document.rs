use std::{ ffi::OsString, fmt::Error, path::PathBuf};

use calamine::{Reader, SheetVisible, open_workbook_auto};

pub struct Document {
  file_path: PathBuf,
  file_extension: OsString,
  word_count: u32,
  token_usage: u32
}

impl Document {
  pub fn new(file_path: PathBuf) -> Self {
    let file_extension = file_path.extension().unwrap().to_owned();
    Self {
      file_path,
      file_extension,
      word_count: 0,
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

  fn count_word(&self, text: &str) -> u32 {
    let mut count = 0;
    let mut in_word = false;
    
    for c in text.chars() {
      if c.is_whitespace() {
        in_word = false;
      } else if self.is_cjk(c) {
        count += 1;
      } else if c.is_alphanumeric() {
        if !in_word {
          count += 1;
          in_word = true;
        }
      } else {
        in_word = false;
      }
    }
    
    count
  }


  fn load_worksheet(&mut self) {
    let mut workbook = open_workbook_auto(&self.file_path).expect("Cannot open file");

    let sheet_names: Vec<String> = workbook.sheets_metadata()
      .iter()
      .filter(|sheet| sheet.visible == SheetVisible::Visible)
      .map(|sheet| sheet.name.clone())
      .collect();
    
    for sheet_name in sheet_names {
      let Ok(range) = workbook.worksheet_range(&sheet_name) else { continue };
      
      self.word_count += range.rows()
      .flat_map(|row| row.iter())
      .filter_map(|cell| match cell {
        calamine::Data::String(text) => Some(self.count_word(text)),
        _ => None
      })
      .sum::<u32>();
    }

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