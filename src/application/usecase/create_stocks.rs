use crate::domain::entities::stocks::Stocks;
use crate::domain::repositories::stocks_repository::StocksRepository;
use crate::application::dtos::stocks_dto::StocksDTO;
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

    pub fn create_stocks(&mut self) -> Result<(), String> {
        let api_key = env::var("FMP_API_KEY").unwrap().to_owned();
        let url = format!("https://financialmodelingprep.com/api/v3/stock/list?apikey={}", api_key.to_owned());
        let response = reqwest::blocking::get(&url).unwrap();

        let stocks_dto: Vec<StocksDTO> = response.json().unwrap();

        let mut stocks_all = self.stocks_repository.find_all().unwrap();

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

            let price_decimal = f64_to_bigdecimal(stock.price.unwrap_or(0.0));
            let new_stock = Stocks {
                symbol: stock.symbol,
                name: stock.name.unwrap_or("".to_string()),
                price: price_decimal,
                exchange: stock.exchange.unwrap_or("".to_string()),
                exchange_short_name: stock.exchangeShortName.unwrap_or("".to_string()),
                is_etf: stock.r#type.unwrap_or("".to_string()) == "etf",
            };

            new_stocks.push(new_stock);
        }

        let _res = self.stocks_repository.create_stocks(new_stocks);

        Ok(())
    }
}
