use crate::domain::repositories::stocks_repository::StocksRepository;
use crate::application::dtos::company_profile::CompanyProfileDTO;
use std::env;
use tokio::task;
use tokio::task::JoinHandle;
pub struct UpdateCompanyService {
    stocks_repository: Box<dyn StocksRepository>,
}

const TASK_LIMIT : usize = 100;
const WAIT_TIME : u64 = 1;

impl UpdateCompanyService {
    pub fn new(stocks_repository: Box<dyn StocksRepository>) -> Self {
        UpdateCompanyService {
            stocks_repository,
        }
    }

    // update_company_info is a method that fetches company information from the FMP API
    pub async fn update_company_info(&mut self) -> Result<(), String> {
        let api_key = env::var("FMP_API_KEY").unwrap().to_owned();
        let stocks_all = self.stocks_repository.find_all().unwrap();

        let mut urls: Vec<String> = Vec::new();

        for stock in stocks_all {
            urls.push(format!("https://financialmodelingprep.com/api/v3/profile/{}?apikey={}", stock.symbol, api_key.to_owned()));
        }

        let mut handles: Vec<JoinHandle<Result<(), String>>> = Vec::new();

        for url in urls {
            let handle = task::spawn(async move {
                let response = reqwest::get(&url).await.unwrap().json();

                let company_profile_dto: Vec<CompanyProfileDTO> = response.await.unwrap();
                let company_profile_domain = company_profile_dto[0].to_domain();

                println!("{:?}", company_profile_domain);

                Ok(())
            });

            handles.push(handle);

            if handles.len() == TASK_LIMIT {

                self.wait_and_execute(&mut handles).await?;

            }
        }

        self.wait_and_execute(&mut handles).await?;

        Ok(())
    }

    // wait_and_execute is a method that waits for the tasks to finish and executes them
    async fn wait_and_execute(&self, handles: &mut Vec<JoinHandle<Result<(), String>>>) -> Result<(), String> {
        tokio::time::sleep(tokio::time::Duration::from_secs(WAIT_TIME)).await;

        let mut tasks = handles.drain(..);
        while let Some(handle) = tasks.next() {
            let res = handle.await.unwrap();
            if res.is_err() {
                return Err("Error".to_string());
            }
        }

        Ok(())
    }

}
