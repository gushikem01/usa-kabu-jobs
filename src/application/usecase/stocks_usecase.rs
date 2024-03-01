use crate::application::dtos::stocks_dto::StocksDTO;
use crate::domain::repositories::stocks_repository::StocksRepository;

pub struct StocksService {
    stocks_repository: Box<dyn StocksRepository>,
}

impl StocksService {
    pub fn new(stocks_repository: Box<dyn StocksRepository>) -> Self {
        StocksService {
            stocks_repository,
        }
    }

    pub fn create_stocks(&mut self) -> Result<(), String> {
        let stocks = StocksDTO {
            symbol: "MSFT".to_string(),
            name: "Microsoft".to_string(),
            exchange: "NASDAQ".to_string(),
            exchange_short_name: "Nasdaq".to_string(),
            is_etf: false,
        };

        let _res = self.stocks_repository.create_stocks(stocks);

        Ok(())
    }

}
