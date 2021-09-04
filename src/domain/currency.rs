use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Currency {
  USD,
  JPY,
}

impl fmt::Display for Currency {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
