#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Hash)]
pub enum PaymentType {
  Large,
  Medium,
  Small,
}
