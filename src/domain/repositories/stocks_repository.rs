use crate::domain::entities::stocks::Stocks as StocksDomain;

#[allow(unused_imports)]
pub trait StocksRepository {
    fn create_stocks(&mut self, stocks: Vec<StocksDomain>) -> Result<(), String>;
    fn find_all(&mut self) -> Result<Vec<StocksDomain>, String>;
}
