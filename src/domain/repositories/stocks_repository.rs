use crate::domain::entities::stocks::{Exchange, Stocks};

#[allow(unused_imports)]
pub trait StocksRepository {
    fn create_stocks(&mut self, stocks: Vec<Stocks>) -> Result<(), String>;
    fn find_all(&mut self) -> Result<Vec<Stocks>, String>;
}
