use bigdecimal::BigDecimal;

#[derive(Debug, Clone)]
pub struct Stocks {
    pub symbol: String,
    pub name: String,
    pub price: BigDecimal,
    pub exchange: String,
    pub exchange_short_name: String,
    pub market_cap: BigDecimal,
    pub is_etf: bool,
    pub is_active: bool
}

impl Stocks {
    pub fn new(symbol: String,
            name: String,
            price: BigDecimal,
            exchange: String,
            exchange_short_name: String,
            market_cap: BigDecimal,
            is_etf: bool,
            is_active: bool) -> Self {
        Stocks {
            symbol,
            name,
            price,
            exchange,
            exchange_short_name,
            market_cap,
            is_etf,
            is_active
        }
    }

}
