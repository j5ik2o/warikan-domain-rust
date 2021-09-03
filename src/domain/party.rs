use domain::party_name::PartyName;

#[derive(Debug)]
pub struct Party {
  name: PartyName,
}

impl Party {
  fn new(n: PartyName) -> Self {
    Party { name: n }
  }
}
