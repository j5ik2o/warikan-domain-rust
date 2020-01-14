#[derive(Debug)]
pub struct PartyName(String);

impl PartyName {

    fn len(&self) -> usize {
        self.0.len()
    }

    fn new(s: &str) -> Self {
        assert!((s.len() as i64) < 255);
        PartyName(s.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn construct_test() {
        let s: PartyName = PartyName::new("1");
        assert_eq!(s.len(), 1);
    }

}