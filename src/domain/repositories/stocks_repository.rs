use crate::application::dtos::stocks_dto::StocksDTO;

#[allow(unused_imports)]
pub trait StocksRepository {
    fn create_stocks(&mut self, stocks: StocksDTO) -> Result<(), String>;
}

