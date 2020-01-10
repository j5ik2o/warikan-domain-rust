use bigdecimal::{BigDecimal, One, Zero};
use std::ops::{Add, Mul, Div, Rem};
use domain::currency::Currency;
use std::cmp::Ordering;

#[derive(Debug,PartialEq)]
pub struct Money {
    currency: Currency,
    amount: BigDecimal,
}

pub trait IMoney<T> {
    fn of(_amount: T, _currency: Currency) -> Self;
}

impl Money {
    pub fn one(currency: Currency) -> Self {
        Self::of(BigDecimal::one(), currency)
    }

    pub fn zero(currency: Currency) -> Self {
        Self::of(BigDecimal::zero(), currency)
    }

    pub fn negated(self) -> Self {
        Self::of(self.amount.mul(BigDecimal::from(-1)), self.currency)
    }

    pub fn add(self, other: Money) -> Self {
        if self.currency != other.currency {
            panic!("Invalid the currency of other")
        }
        let result = self.amount.add(other.amount);
        Self::of(result, other.currency)
    }

    pub fn sub(self, other: Money) -> Self {
        self.add(other.negated())
    }

    pub fn mul(self, n: BigDecimal) -> Self {
        Self::of(self.amount.mul(n), self.currency)
    }

    pub fn div(self, n: BigDecimal) -> Self {
        Self::of(self.amount.div(n), self.currency)
    }

    pub fn rem(self, n: BigDecimal) -> Self {
        Self::of(self.amount.rem(n), self.currency)
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.currency != other.currency {
            panic!("Invalid the currency of other")
        }
        self.amount.partial_cmp(&other.amount)
    }
}

impl IMoney<i64> for Money {
    fn of(_amount: i64, _currency: Currency) -> Self {
        Money {
            amount: BigDecimal::from(_amount),
            currency: _currency,
        }
    }
}

impl IMoney<BigDecimal> for Money {
    fn of(_amount: BigDecimal, _currency: Currency) -> Self {
        Money {
            amount: _amount,
            currency: _currency,
        }
    }
}
