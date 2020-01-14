use im::Vector;

#[derive(Debug, Clone, PartialEq)]
pub enum SecretaryType {
    NonSecretary,
    Secretary,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberName(String);

impl MemberName {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn new(s: &str) -> Self {
        assert!((s.len() as i64) < 255);
        MemberName(s.to_string())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    name: MemberName,
    secretary_type: SecretaryType,
}

impl Member {
    fn new(n: MemberName, st: SecretaryType) -> Self {
        Member {
            name: n,
            secretary_type: st,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Members(Vector<Member>);


impl Members {

    fn unit(head: &Member) -> Self {
        Self::new(head, &[])
    }

    fn new(head: &Member, tail: &[Member]) -> Self {
        let mut members =  Vector::unit(head.clone());
        members.append(Vector::from(tail));
        Members(members)
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn contains(&self, element: &Member) -> bool {
        self.0.contains(element)
    }

    fn exists<F>(&self, f: F) -> bool where F: Fn(&Member) -> bool {
        self.0.iter().any(|x| f(x))
    }

    fn size_of_secretaries(&self) -> usize {
        self.0.iter().filter(|&x| x.secretary_type == SecretaryType::Secretary).count()
    }

    fn size_of_non_secretaries(&self) -> usize {
        self.0.iter().filter(|&x| x.secretary_type == SecretaryType::NonSecretary).count()
    }

    fn size_of(&self, st: SecretaryType) -> usize {
        if st == SecretaryType::Secretary {
            self.size_of_secretaries()
        } else {
            self.size_of_non_secretaries()
        }
    }

    fn secretaries(&self) -> Self {
        Self(self.0.iter().filter(|&x| x.secretary_type == SecretaryType::Secretary).cloned().collect())
    }

    fn non_secretaries(&self) -> Self {
        Self(self.0.iter().filter(|&x| x.secretary_type == SecretaryType::NonSecretary).cloned().collect())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_exists() {
        let mn = MemberName::new("---");
        let m = Member::new(mn, SecretaryType::NonSecretary);
        let ms = Members::unit(&m);
        assert_eq!(ms.size(), 1);
        let b = ms.exists(|x| x == &m);
        assert!(b);
    }
}