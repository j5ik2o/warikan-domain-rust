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
  let party = Party::new(PartyName::new("test")).with_members(members);
  let warikan = party.warikan(BillingAmount::new(Money::one(Currency::JPY)));
  println!("{:?}", warikan);
}
