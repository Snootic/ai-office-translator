use crate::Document;

use docx_rs::*;

pub trait Doc {
  fn process_paragraph(&mut self, paragraph: Paragraph) -> Vec<String>;
  fn load(&mut self);
}

impl Doc for Document {
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

  fn load(&mut self) {
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
}