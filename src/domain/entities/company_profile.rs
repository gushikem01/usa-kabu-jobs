use bigdecimal::BigDecimal;

#[derive(Debug, Clone)]
pub struct CompanyProfile {
    pub symbol: String,
    pub price: BigDecimal,
    pub market_cap: BigDecimal,
}
