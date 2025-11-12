use std::{ ffi::OsString, fmt::Error, path::PathBuf};

use docx_rs::*;
use calamine::{Reader, SheetVisible, open_workbook_auto};
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

    self.get_dates_metadata();
    self.calculate_tokens(all_words);
  }

  fn process_paragraph(& mut self, paragraph: Paragraph) -> Vec<String> {
    let texts: Vec<String> = paragraph.children
      .into_iter()
      .filter_map(|child| match child {
          ParagraphChild::Run(run) => Some(run),
          _ => None,
      })
      .flat_map(|run| {
        run.children.into_iter().filter_map(|child| match child {
          RunChild::Text(text) => Some(text.text),
            _ => None,
        })
      })
      .collect();
    
    let mut all_words = Vec::new();
    for text in texts {
      let (words, count) = self.count_word(&text);
      self.word_count += count;
      all_words.extend(words);
    }
    
    all_words
  }

  fn load_doc(&mut self) {
    
    let file = std::fs::read(&self.file_path).expect("Cannot read file");
    
    let reader = read_docx(&file).unwrap();

    let mut all_words: Vec<String> = Vec::new();

    for child in reader.document.children {
      match child {
         DocumentChild::Paragraph(paragraph) => {
          all_words.extend(self.process_paragraph(*paragraph));
        },
        DocumentChild::Table(tab) => {
          tab.rows.into_iter()
          .filter_map(|child| match child {
            TableChild::TableRow(row) => Some(row)
          })
          .flat_map(|row| row.cells.into_iter())
          .filter_map(|child| match child {
            TableRowChild::TableCell(cell) => Some(cell)
          })
          .flat_map(|cell| cell.children.into_iter())
          .filter_map(|child| match child {
            TableCellContent::Paragraph(paragraph) => Some(paragraph),
              _ => None,
          })
          .for_each(|paragraph| {
            all_words.extend(self.process_paragraph(paragraph));
          });
        },
        _ => {}
      }
    }
    
    self.get_dates_metadata();
    self.calculate_tokens(all_words);
  }

  pub fn load(&mut self) -> Result<(), Error> {
    match self.file_extension.to_str() {
      Some("xlsx") => {
        self.load_worksheet();
        Ok(())
      },
      Some("doc") | Some("docx") => {
        self.load_doc();
        Ok(())
      }
      _ => Err(Error)
    }
  }
}