use std::collections::HashMap;

use domain::amount::{
  BillingAmount, PartyPaymentTypeRatios, PaymentAmount, PaymentTotalAmount, WeightedSum,
};

pub use self::currency::*;
pub use self::member::*;
pub use self::money::*;
pub use self::party_name::*;

pub mod amount;
mod currency;
mod member;
mod money;
mod party_name;
pub mod payment;

#[derive(Debug, Clone)]
pub struct Warikan {
  result: HashMap<Member, PaymentAmount>,
}

impl Warikan {
  pub fn new(result: HashMap<Member, PaymentAmount>) -> Self {
    Self { result }
  }

  pub fn result(&self) -> HashMap<Member, PaymentAmount> {
    self.result.clone()
  }

  pub fn payment_amount_by(&self, member: Member) -> Option<&PaymentAmount> {
    self.result.get(&member)
  }

  pub fn payment_amount_by_member_name(
    &self,
    member_name: MemberName,
  ) -> Option<(Member, PaymentAmount)> {
    self
      .result
      .clone()
      .into_iter()
      .find(|(k, _v)| k.name == member_name)
  }

  pub fn total_amount(&self, currency: Currency) -> PaymentTotalAmount {
    let init = Money::zero(currency);
    let money = self
      .result
      .values()
      .clone()
      .into_iter()
      .cloned()
      .fold(init, |r, e| r + e.0);
    PaymentTotalAmount(money)
  }
}

pub struct Party {
  name: PartyName,
  members_opt: Option<Members>,
  party_payment_type_ratios: PartyPaymentTypeRatios,
}

impl Party {
  pub fn new(name: PartyName) -> Self {
    Self {
      name,
      members_opt: None,
      party_payment_type_ratios: PartyPaymentTypeRatios::default(),
    }
  }

  pub fn with_members(mut self, members: Members) -> Self {
    self.members_opt = Some(members);
    self
  }

  pub fn with_party_payment_type_ratios(
    mut self,
    party_payment_type_ratios: PartyPaymentTypeRatios,
  ) -> Self {
    self.party_payment_type_ratios = party_payment_type_ratios;
    self
  }

  pub fn warikan(&self, billing_amount: BillingAmount) -> Warikan {
    let members = self.members_opt.as_ref().unwrap();
    let weighted_sum = self.weighted_sum();
    let payment_base_amount = billing_amount / weighted_sum;
    let result = members
      .payment_types()
      .iter()
      .map(|(member, payment_type)| {
        let payment_ratio = self
          .party_payment_type_ratios
          .payment_type_ratio(payment_type);
        (
          member.clone(),
          payment_base_amount.clone() * payment_ratio.clone(),
        )
      })
      .collect::<HashMap<_, _>>();
    Warikan::new(result)
  }

  fn weighted_sum(&self) -> WeightedSum {
    let members = self.members_opt.as_ref().unwrap();
    let payment_ratios = members
      .0
      .iter()
      .map(|member| {
        self
          .party_payment_type_ratios
          .payment_type_ratio(&member.payment_type)
          .clone()
      })
      .collect::<Vec<_>>();
    let (first, tail) = payment_ratios.split_first().unwrap();
    WeightedSum::from(first.clone(), tail)
  }
}
