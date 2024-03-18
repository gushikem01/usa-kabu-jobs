use crate::domain::repositories::stocks_repository::StocksRepository;
use crate::application::dtos::stocks_dto::{CompanyProfileDTO, StocksDTO};
use std::env;
use tokio::task;
use tokio::task::JoinHandle;
pub struct UpdateCompanyService {
    stocks_repository: Box<dyn StocksRepository>,
}

const TASK_LIMIT : usize = 5;

impl UpdateCompanyService {
    pub fn new(stocks_repository: Box<dyn StocksRepository>) -> Self {
        UpdateCompanyService {
            stocks_repository,
        }
    }

    /// Updates the company information
    pub async fn update_company_info(&mut self) -> Result<(), String> {
        let api_key = env::var("FMP_API_KEY").unwrap().to_owned();

        let stocks_all = self.stocks_repository.find_all().unwrap();

        let mut urls: Vec<String> = Vec::new();

        for stock in stocks_all {
            urls.push(format!("https://financialmodelingprep.com/api/v3/profile/{}?apikey={}", stock.symbol, api_key.to_owned()));
        }

        let mut handles: Vec<JoinHandle<Result<(), String>>> = Vec::new();

        let mut count = 0;
        for url in urls {
            count += 1;

            println!("Fetching count:{} url:{} len:{}", count, url, handles.len());

            let handle = task::spawn(async move {
                let response = reqwest::get(&url).await.unwrap().text().await.unwrap();

                let _company_profile_dto: Vec<CompanyProfileDTO> = serde_json::from_str(&response).unwrap();
                println!("{}", response);

                Ok(())
            });

            handles.push(handle);

            if handles.len() == TASK_LIMIT {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                let mut tasks = handles.drain(..);

                while let Some(handle) = tasks.next() {
                    let res = handle.await.unwrap();
                    if res.is_err() {
                        return Err("Error".to_string());
                    }
                }

            }

        }

        Ok(())
    }
}
