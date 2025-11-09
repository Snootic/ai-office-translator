use serde::Serialize;

use crate::structs::dictionary::*;

#[derive(Serialize)]
#[derive(Debug)]
pub struct GlossaryType<'a> {
  name: &'a str,
  dictionaries: Vec<Dictionary>
}

impl<'a> GlossaryType<'a> {
  pub fn new(name: &'a str, dictionaries: Vec<Dictionary>) -> Self {
    Self {
      name,
      dictionaries
    }
  }
}