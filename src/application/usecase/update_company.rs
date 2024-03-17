use crate::domain::repositories::stocks_repository::StocksRepository;

pub struct UpdateCompanyService {
    stocks_repository: Box<dyn StocksRepository>,
}

impl UpdateCompanyService {
    pub fn new(stocks_repository: Box<dyn StocksRepository>) -> Self {
        UpdateCompanyService {
            stocks_repository,
        }
    }

    pub fn update_company_info(&mut self) -> Result<(), String> {
        // TODO
        println!("Stocks: {:?}", "".to_string());
        Ok(())
    }
}
