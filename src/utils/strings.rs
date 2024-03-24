use bigdecimal::BigDecimal;
use std::str::FromStr;

pub fn f64_to_bigdecimal(num: f64) -> BigDecimal {
    BigDecimal::from_str(&num.to_string()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_to_bigdecimal() {
        let num = 10.0;
        let expected = BigDecimal::from_str("10").unwrap();
        let result = f64_to_bigdecimal(num);

        assert_eq!(expected, result);
    }
}
