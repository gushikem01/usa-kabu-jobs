#[derive(Debug, Clone)]
pub struct Stocks {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub exchange_short_name: String,
    pub is_etf: bool
}
