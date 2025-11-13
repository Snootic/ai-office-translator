pub type LanguageName = String;
pub type LanguageCode = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Language {
  name: LanguageName,
  code: LanguageCode,
}

impl Language {
  pub fn new(name: LanguageName, code: LanguageCode) -> Self {
    Self {
      name,
      code
    }
  }
}