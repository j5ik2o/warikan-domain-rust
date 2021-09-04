use std::fmt::Formatter;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Hash)]
pub enum PaymentType {
  Large,
  Medium,
  Small,
}

impl std::fmt::Display for PaymentType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}