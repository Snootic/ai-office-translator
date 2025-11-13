use serde::Serialize;

use crate::structs::language::LanguageCode;

#[derive(Serialize)]
#[derive(Debug)]
pub struct Dictionary {
  source_lang: LanguageCode,
  target_lang: LanguageCode,
  entries: String,
  entries_format: String,
}

impl Dictionary {
  pub fn new(source_lang: LanguageCode, target_lang: LanguageCode, entries: String, entries_format: String) -> Self {
    Self {
      source_lang,
      target_lang,
      entries,
      entries_format,
    }
  }
}