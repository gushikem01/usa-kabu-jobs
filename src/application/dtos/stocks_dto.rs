use crate::domain::entities::stocks::Stocks;

#[derive(Debug)]
pub struct StocksDTO {
    pub symbol: String,
    pub name : String,
    pub exchange: String,
    pub exchange_short_name: String,
    pub is_etf: bool,
}

impl From<Stocks> for StocksDTO {
    fn from(stocks: Stocks) -> Self {
        StocksDTO {
            symbol: stocks.symbol.clone(),
            name: stocks.name.clone(),
            exchange: stocks.exchange.clone(),
            exchange_short_name: stocks.exchange_short_name.clone(),
            is_etf: stocks.is_etf,
        }
    }
}

