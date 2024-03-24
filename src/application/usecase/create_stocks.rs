use crate::domain::entities::stocks::Stocks;
use crate::domain::repositories::stocks_repository::StocksRepository;
use crate::application::dtos::stocks::StocksDTO;
use crate::utils::strings::f64_to_bigdecimal;
use std::env;

pub struct StocksService {
    stocks_repository: Box<dyn StocksRepository>,
}

impl StocksService {
    pub fn new(stocks_repository: Box<dyn StocksRepository>) -> Self {
        StocksService {
            stocks_repository,
        }
    }

    // create_stocks is a method that fetches stock information from the FMP API
    pub async fn create_stocks(&mut self) -> Result<(), String> {
        let api_key = env::var("FMP_API_KEY").unwrap().to_owned();
        let url = format!("https://financialmodelingprep.com/api/v3/stock/list?apikey={}", api_key.to_owned());

        let response = reqwest::get(&url).await.unwrap().text().await.unwrap();

        let mut stocks_all: Vec<Stocks> = self.stocks_repository.find_all().unwrap();

        let stocks_dto: Vec<StocksDTO> = serde_json::from_str(&response).unwrap();

        let mut new_stocks: Vec<Stocks> = Vec::new();
        for stock in stocks_dto {

            let mut exists = false;

            for stock_exist in stocks_all.iter() {
                if stock_exist.symbol == stock.symbol {
                    exists = true;
                    stocks_all.remove(stocks_all.iter().position(|x| x.symbol == stock.symbol).unwrap());
                    break;
                }
            }

            if exists {
                continue;
            }

            let new_stock = Stocks::new(
                stock.symbol,
                stock.name.unwrap_or("".to_string()),
                f64_to_bigdecimal(stock.price.unwrap_or(0.0)),
                stock.exchange.unwrap_or("".to_string()),
                stock.exchangeShortName.unwrap_or("".to_string()),
                f64_to_bigdecimal(0 as f64),
                stock.r#type.unwrap_or("".to_string()) == "etf",
                true);

            new_stocks.push(new_stock);
        }

        let _res = self.stocks_repository.create_stocks(new_stocks);

        Ok(())
    }
}
