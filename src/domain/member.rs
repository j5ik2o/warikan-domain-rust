use std::fmt::Formatter;
use domain::payment::PaymentType;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SecretaryType {
  NonSecretary,
  Secretary,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemberName(String);

impl std::fmt::Display for MemberName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl MemberName {
  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn new(s: &str) -> Self {
    assert!((s.len() as i64) < 255);
    Self(s.to_string())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Member {
  pub name: MemberName,
  secretary_type: SecretaryType,
  pub payment_type: PaymentType,
}

impl Member {
  pub fn new(n: MemberName, st: SecretaryType, pt: PaymentType) -> Self {
    Member {
      name: n,
      secretary_type: st,
      payment_type: pt,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Members(pub(crate) Vec<Member>);

impl Members {
  pub fn unit(head: Member) -> Self {
    Self::new(head, &[])
  }

  pub fn new(head: Member, tail: &[Member]) -> Self {
    let mut members = vec![head];
    members.extend_from_slice(tail);
    Members(members)
  }

  pub fn payment_types(&self) -> HashMap<Member, PaymentType> {
    self
      .0
      .iter()
      .map(|e| (e.clone(), e.clone().payment_type))
      .collect::<HashMap<_, _>>()
  }

  fn size(&self) -> usize {
    self.0.len()
  }

  fn contains(&self, element: &Member) -> bool {
    self.0.contains(element)
  }

  fn exists<F>(&self, f: F) -> bool
  where
    F: Fn(&Member) -> bool,
  {
    self.0.iter().any(|x| f(x))
  }

  fn size_of_secretaries(&self) -> usize {
    self
      .0
      .iter()
      .filter(|&x| x.secretary_type == SecretaryType::Secretary)
      .count()
  }

  fn size_of_non_secretaries(&self) -> usize {
    self
      .0
      .iter()
      .filter(|&x| x.secretary_type == SecretaryType::NonSecretary)
      .count()
  }

  fn size_of(&self, st: SecretaryType) -> usize {
    if st == SecretaryType::Secretary {
      self.size_of_secretaries()
    } else {
      self.size_of_non_secretaries()
    }
  }

  fn secretaries(&self) -> Self {
    Self(
      self
        .0
        .iter()
        .filter(|&x| x.secretary_type == SecretaryType::Secretary)
        .cloned()
        .collect(),
    )
  }

  fn non_secretaries(&self) -> Self {
    Self(
      self
        .0
        .iter()
        .filter(|&x| x.secretary_type == SecretaryType::NonSecretary)
        .cloned()
        .collect(),
    )
  }

  fn as_slice(&self) -> &[Member] {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn member_exists() {
    let mn = MemberName::new("---");
    let m = Member::new(mn, SecretaryType::NonSecretary, PaymentType::Medium);
    let ms = Members::unit(m.clone());
    assert_eq!(ms.size(), 1);
    let b = ms.exists(move |x| *x == m);
    let s = ms.as_slice();
    println!("{:?}", s);
    assert!(b);
  }
}
