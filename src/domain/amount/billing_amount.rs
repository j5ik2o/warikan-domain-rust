use std::ops::{Add, Div, Sub};

use domain::Money;
use domain::amount::{PaymentTotalAmount, WeightedSum, PaymentAmount};
use bigdecimal::BigDecimal;

#[derive(Debug, Clone)]
pub struct BillingAmount(Money);

impl BillingAmount {
  pub fn new(value: Money) -> Self {
    Self(value)
  }
}

impl Add for BillingAmount {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}

impl Sub<PaymentTotalAmount> for BillingAmount {
  type Output = Self;

  fn sub(self, rhs: PaymentTotalAmount) -> Self::Output {
    Self(self.0 - rhs.0)
  }
}

impl Div<WeightedSum> for BillingAmount {
  type Output = PaymentAmount;

  fn div(self, rhs: WeightedSum) -> Self::Output {
    PaymentAmount::new(self.0.div(BigDecimal::from(rhs.0)))
  }
}
