use calamine::{Reader, SheetVisible, open_workbook_auto};

use crate::Document;

pub trait Xls {
  fn load(&mut self);
}

impl Xls for Document {
  fn load(&mut self) {
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
}