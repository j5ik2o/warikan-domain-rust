extern crate example;
extern crate bigdecimal;

use example::domain::currency::Currency;
use example::domain::money::*;
use bigdecimal::BigDecimal;

fn main() {
  let b1 = BigDecimal::from(1);
  let jpy1 = Currency::JPY;
  let m1 = Money::of(b1, jpy1);

  let b2 = BigDecimal::from(1);
  let jpy2 = Currency::JPY;
  let m2 = Money::of(b2, jpy2);

  let m3 = m1.add(m2);
  println!("m = {:?}", m3);

  if m3 >= Money::one(Currency::JPY) {
    println!("yes")
  } else {
    println!("no")
  }
}
