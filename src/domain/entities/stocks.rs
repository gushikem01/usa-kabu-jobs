use bigdecimal::BigDecimal;

#[derive(Debug, Clone)]
pub struct Stocks {
    pub symbol: String,
    pub name: String,
    pub price: BigDecimal,
    pub exchange: String,
    pub exchange_short_name: String,
    pub is_etf: bool
}
