use std::ops::Mul;

use domain::Money;
use domain::amount::PaymentRatio;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct PaymentAmount(pub(crate) Money);

impl std::fmt::Display for PaymentAmount {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "({})", self.0)
  }
}

impl PaymentAmount {
  pub fn new(value: Money) -> Self {
    assert!(value.is_non_zero());
    Self(value)
  }
}

impl Mul<PaymentRatio> for PaymentAmount {
  type Output = Self;

  fn mul(self, rhs: PaymentRatio) -> Self::Output {
    Self(self.0 * rhs)
  }
}
