use domain::Money;
use std::ops::{Add, Sub, Div};

pub struct BillingAmount(Money);

impl Add for BillingAmount {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}

impl Sub for BillingAmount {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0 - rhs.0)
  }
}

impl Div for BillingAmount {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    self.0.div(rhs.0.)
  }
}
