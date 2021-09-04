use std::ops::Add;
use domain::amount::PaymentRatio;

#[derive(Debug, Clone)]
pub struct WeightedSum(pub(crate) f64);

impl WeightedSum {
  pub fn from(head: PaymentRatio, tail: &[PaymentRatio]) -> Self {
    let init = Self::new(head.0);
    let values = Vec::from(tail);
    values.into_iter().fold(init, |w, p| w + p)
  }
  pub fn new(value: f64) -> Self {
    assert!(value >= 0.0);
    Self(value)
  }
}

impl Add<PaymentRatio> for WeightedSum {
  type Output = Self;

  fn add(self, rhs: PaymentRatio) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}
