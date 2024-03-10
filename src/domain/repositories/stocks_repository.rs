use crate::domain::entities::stocks::Stocks;

#[allow(unused_imports)]
pub trait StocksRepository {
    fn create_stocks(&mut self, stocks: Vec<Stocks>) -> Result<(), String>;
}

