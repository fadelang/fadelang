use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

pub struct Source {
  path: PathBuf,
}

impl Source {
  //pub fn get_path(&self) -> PathBuf {
  //  self.path
  //}

  pub fn as_file(&self) -> std::io::Result<File> {
    File::open(&self.path)
  }

  pub fn buf_reader(&self) -> BufReader<File> {
    BufReader::new(self.as_file().unwrap())
  }

  pub fn read_to_string(&self) -> String {
    let mut buf = String::default();
    self.as_file().unwrap().read_to_string(&mut buf).unwrap();
    buf
  }
}

impl From<&str> for Source {
  fn from(source: &str) -> Self {
    Self {
      path: PathBuf::from(source),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_file() -> std::io::Result<()> {
    let source = Source::from("test/source/tests/from_file.fl");

    assert_eq!(
      source.read_to_string(),
      "main(): -> u8 := {\n  return 0;\n}\n"
    );
    Ok(())
  }
}
