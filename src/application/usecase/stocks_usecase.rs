use crate::domain::entities::stocks::Stocks;
use crate::domain::repositories::stocks_repository::StocksRepository;
use crate::application::dtos::stocks_dto::{StocksDTO};
use std::env;
use bigdecimal::BigDecimal;
use std::str::FromStr;

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

        let stocks: Vec<StocksDTO> = response.json().unwrap();

        let stocks: Vec<Stocks> = stocks.into_iter().map(|stock: StocksDTO| {

            let price_decimal = f64_to_bigdecimal(stock.price.unwrap_or(0.0));

            Stocks {
                symbol: stock.symbol,
                name: stock.name.unwrap_or("".to_string()),
                price: price_decimal,
                exchange: stock.exchange.unwrap_or("".to_string()),
                exchange_short_name: stock.exchangeShortName.unwrap_or("".to_string()),
                is_etf: stock.r#type.unwrap_or("".to_string()) == "etf",
            }
        }).collect();

        let _res = self.stocks_repository.create_stocks(stocks);

        Ok(())
    }

}

fn f64_to_bigdecimal(num: f64) -> BigDecimal {
    BigDecimal::from_str(&num.to_string()).unwrap()
}
