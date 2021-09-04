use std::fmt;

#[derive(Debug, Clone)]
pub struct PartyName(String);

impl fmt::Display for PartyName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl PartyName {
  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn new(s: &str) -> Self {
    assert!((s.len() as i64) < 255);
    Self(s.to_string())
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn construct_test() {
    let s: PartyName = PartyName::new("1");
    assert_eq!(s.len(), 1);
  }
}
