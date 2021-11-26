use std::fmt::{Display, Formatter};

use crate::source::Source;
use crate::token::*;

pub struct Tokenizer {
  caret_pos: CaretPos,
}

impl Tokenizer {
  pub fn tokenize(&mut self, source: &Source) -> std::io::Result<Vec<Box<dyn Token>>> {
    let source = source.read_to_string();
    let mut tokens = Vec::<Box<dyn Token>>::new();

    let mut chars = source.chars().peekable();

    loop {
      let char_cur = chars.next();

      self.caret_pos.process_char(char_cur);

      if char_cur == None {
        tokens.push(Box::new(EndOfFile::default()));
        break;
      } else if let Some(char_cur) = char_cur {
        if char_cur == '(' {
          tokens.push(Box::new(Parenthesis::open()));
        } else if char_cur == ')' {
          tokens.push(Box::new(Parenthesis::close()));
        } else if char_cur == '<' {
          tokens.push(Box::new(Operator::from(OperatorType::GenericBlockBegin)));
        } else if char_cur == '>' {
          tokens.push(Box::new(Operator::from(OperatorType::GenericBlockEnd)));
        } else if char_cur == ';' {
          tokens.push(Box::new(Operator::from(OperatorType::StatementTerminator)));
        } else if char_cur == ':' {
          tokens.push(Box::new(Operator::from(OperatorType::TypeSpecifier)));
        } else if char_cur == ',' {
          tokens.push(Box::new(Operator::from(OperatorType::CommaSeparator)));
        } else if char_cur == '-' && chars.peek().unwrap() == &'>' {
          chars.next().unwrap();
          tokens.push(Box::new(Operator::from(OperatorType::ReturnType)));
        } else if char_cur == '+' {
          tokens.push(Box::new(Operator::from(OperatorType::Addition)));
        } else if char_cur == '{' {
          tokens.push(Box::new(Brace::open()));
        } else if char_cur == '}' {
          tokens.push(Box::new(Brace::close()));
        } else if char_cur == ' ' {
          while let Some(' ') = chars.peek() {
            chars.next();
          }
          tokens.push(Box::new(Whitespace::default()));
        } else if char_cur == '\n' {
          tokens.push(Box::new(NewLine::default()));
        } else if Identifier::is_valid_char(&char_cur, true) || Keyword::is_valid_char(&char_cur) {
          let mut buf = String::from(char_cur);

          while let Some(peek) = chars.peek() {
            if Identifier::is_valid_char(peek, buf.is_empty())
              || Keyword::is_valid_char(peek)
            {
              buf.push(chars.next().unwrap());
            } else {
              break;
            }
          }

          if !buf.is_empty() {
            if is_keyword(&buf) {
              tokens.push(Box::new(Keyword::from(buf.clone())));
            } else {
              tokens.push(Box::new(Identifier::from(buf.clone())));
            }
          }
        } else {
          panic!("Character '{}' was not handled", char_cur);
        }
      }
    }

    Ok(tokens)
  }

  pub fn get_caret_pos(&self) -> CaretPos {
    self.caret_pos
  }
}

impl Default for Tokenizer {
  fn default() -> Self {
    Self {
      caret_pos: CaretPos::default(),
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaretPos {
  line: usize,
  column: usize,
}

impl CaretPos {
  fn reset_column(&mut self) {
    self.column = 1;
  }

  fn increment_line(&mut self) {
    self.line += 1;
  }

  fn increment_column(&mut self) {
    self.column += 1;
  }

  fn new_line(&mut self) {
    self.increment_line();
    self.reset_column();
  }

  pub fn get_line(&self) -> usize {
    self.line
  }

  pub fn get_column(&self) -> usize {
    self.column
  }

  pub fn process_char(&mut self, c: Option<char>) {
    if c.is_none() {
      return;
    }

    self.increment_column();

    if let Some('\n') = c {
      self.new_line();
    }
  }
}

impl Default for CaretPos {
  fn default() -> Self {
    Self { line: 1, column: 1 }
  }
}

impl From<(usize, usize)> for CaretPos {
  fn from(pos: (usize, usize)) -> Self {
    Self {
      line: pos.0,
      column: pos.1,
    }
  }
}

impl Display for CaretPos {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, ":{}:{}", self.get_line(), self.get_column())
  }
}

#[cfg(test)]
mod tests {
  use crate::source::Source;
  use crate::token::*;
  use crate::tokenizer::{Tokenizer, CaretPos};

  macro_rules! tokens_equal {
    ($expected:expr, $actual:expr) => {
      println!("(expected) > {:?}", $expected);
      println!(" (actual)  > {:?}", $actual);

      assert_eq!($expected.len(), $actual.len());
    };
  }

  #[test]
  fn tokenize_main_fn() {
    let source = Source::from("test/tokenizer/main_fn.fl");
    let mut tokenizer = Tokenizer::default();
    let tokenized = tokenizer.tokenize(&source).unwrap();

    let tokenized_eq: Vec<Box<dyn Token>> = vec![
      Box::new(Identifier::from("main")),
      Box::new(Parenthesis::open()),
      Box::new(Parenthesis::close()),
      Box::new(Operator::from(OperatorType::StatementTerminator)),
      Box::new(NewLine::default()),
      Box::new(EndOfFile::default()),
    ];

    println!("tc > {:?}", tokenized);
    println!("eq > {:?}", tokenized_eq);

    assert_eq!(tokenized.len(), tokenized_eq.len());

    // for i in 0..tokenized.len() {
    //   assert_eq!(tokenized[i], tokenized_eq[i])
    // }
  }

  #[test]
  fn tokenize_other_fn() {
    let source = Source::from("test/tokenizer/other_fn.fl");
    let mut tokenizer = Tokenizer::default();
    let actual = tokenizer.tokenize(&source).unwrap();

    let expected: Vec<Box<dyn Token>> = vec![
      Box::new(Identifier::from("man")),
      Box::new(Parenthesis::open()),
      Box::new(Parenthesis::close()),
      Box::new(Operator::from(OperatorType::StatementTerminator)),
      Box::new(NewLine::default()),
      Box::new(Identifier::from("other_fn")),
      Box::new(Parenthesis::open()),
      Box::new(Parenthesis::close()),
      Box::new(Operator::from(OperatorType::StatementTerminator)),
      Box::new(NewLine::default()),
      Box::new(EndOfFile::default()),
    ];

    tokens_equal!(expected, actual);
  }

  #[test]
  fn tokenize_add() {
    let source = Source::from("test/tokenizer/add.fl");
    let mut tokenizer = Tokenizer::default();
    let actual = tokenizer.tokenize(&source).unwrap();

    let expected: Vec<Box<dyn Token>> = vec![
      Box::new(Identifier::from("add")),
      Box::new(Parenthesis::open()),
      Box::new(Identifier::from("a")),
      Box::new(Operator::from(OperatorType::TypeSpecifier)),
      Box::new(Whitespace::default()),
      Box::new(Keyword::from("u8")),
      Box::new(Operator::from(OperatorType::CommaSeparator)),
      Box::new(Whitespace::default()),
      Box::new(Identifier::from("b")),
      Box::new(Operator::from(OperatorType::TypeSpecifier)),
      Box::new(Whitespace::default()),
      Box::new(Keyword::from("u8")),
      Box::new(Parenthesis::close()),
      Box::new(Operator::from(OperatorType::TypeSpecifier)),
      Box::new(Whitespace::default()),
      Box::new(Operator::from(OperatorType::ReturnType)),
      Box::new(Whitespace::default()),
      Box::new(Keyword::from("u8")),
      Box::new(Whitespace::default()),
      Box::new(Brace::open()),
      Box::new(NewLine::default()),
      Box::new(Whitespace::default()),
      Box::new(Keyword::from("return")),
      Box::new(Whitespace::default()),
      Box::new(Identifier::from("a")),
      Box::new(Whitespace::default()),
      Box::new(Operator::from(OperatorType::Addition)),
      Box::new(Whitespace::default()),
      Box::new(Identifier::from("b")),
      Box::new(Operator::from(OperatorType::StatementTerminator)),
      Box::new(NewLine::default()),
      Box::new(Brace::close()),
      Box::new(Operator::from(OperatorType::StatementTerminator)),
      Box::new(NewLine::default()),
      Box::new(EndOfFile::default()),
    ];

    tokens_equal!(expected, actual);
  }

  #[test]
  fn rows_and_colums() {
    let source = Source::from("test/tokenizer/other_fn.fl");
    let mut tokenizer = Tokenizer::default();
    let _ = tokenizer.tokenize(&source).unwrap();
    let actual = tokenizer.get_caret_pos();
    let expected = CaretPos::from((3, 1));

    assert_eq!(actual, expected)
  }
}
