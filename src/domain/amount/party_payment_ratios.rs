use domain::amount::PaymentRatio;
use domain::payment::PaymentType;
use std::collections::HashMap;

pub struct PartyPaymentTypeRatios {
  values: HashMap<PaymentType, PaymentRatio>,
}

impl PartyPaymentTypeRatios {
  pub fn new(small: PaymentRatio, large: PaymentRatio) -> Self {
    let mut values = HashMap::new();
    values.insert(PaymentType::Large, large);
    values.insert(PaymentType::Medium, PaymentRatio::new(1.00));
    values.insert(PaymentType::Small, small);
    Self { values }
  }
  pub fn payment_type_ratio(&self, t: &PaymentType) -> &PaymentRatio {
    self.values.get(t).unwrap()
  }
}

impl Default for PartyPaymentTypeRatios {
  fn default() -> Self {
    Self::new(PaymentRatio::new(0.8), PaymentRatio::new(1.2))
  }
}
