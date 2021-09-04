extern crate anyhow;
extern crate bigdecimal;

use bigdecimal::BigDecimal;

use domain::*;
use domain::amount::*;
use domain::payment::PaymentType;

mod domain;

fn main() {
  let members = Members::new(
    Member::new(
      MemberName::new("Junichi Kato"),
      SecretaryType::Secretary,
      PaymentType::Large,
    ),
    &[
      Member::new(
        MemberName::new("Test2"),
        SecretaryType::Secretary,
        PaymentType::Small,
      ),
      Member::new(
        MemberName::new("Test3"),
        SecretaryType::Secretary,
        PaymentType::Medium,
      ),
    ],
  );
  let party = Party::new(PartyName::new("test"))
    .with_members(members)
    .with_party_payment_type_ratios(PartyPaymentTypeRatios::new(
      PaymentRatio::new(2.0),
      PaymentRatio::new(0.5),
    ));
  let warikan = party.warikan(BillingAmount::new(Money::of(100000, Currency::JPY)));
  warikan.result().iter().for_each(|(member, payment)| {
    println!("member = {}, payment = {}", member, payment);
  });
}
