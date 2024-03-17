use bigdecimal::BigDecimal;
use std::str::FromStr;

pub fn f64_to_bigdecimal(num: f64) -> BigDecimal {
    BigDecimal::from_str(&num.to_string()).unwrap()
}
