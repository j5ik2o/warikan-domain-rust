/// 支払割合
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PaymentRatio(pub(crate) f64);

impl PaymentRatio {
  pub fn new(value: f64) -> Self {
    assert!(value.is_sign_positive());
    Self(value)
  }
}
