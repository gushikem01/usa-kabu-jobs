#[macro_use]
extern crate diesel;

mod application;
mod domain;
mod infrastructure;
mod schema;
mod utils;

use application::usecase::create_stocks::StocksService;
use application::usecase::update_company::UpdateCompanyService;
use infrastructure::database::establish_connection;
use infrastructure::adapters::stocks_repository::StocksRepositoryImpl;
const ARGS_LENGTH: usize = 2;

fn main () {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < ARGS_LENGTH {
        println!("Please provide a command");
        return;
    }

    for n in args {
        match n.as_str() {
            "create_stocks" => {
                let pool = establish_connection();
                let stocks_repository = StocksRepositoryImpl::new(pool);
                let mut stocks_service = StocksService::new(Box::new(stocks_repository));
                let _result = stocks_service.create_stocks();
                println!("Stocks created");
            },
            "update_company_info" => {
                let pool = establish_connection();
                let stocks_repository = StocksRepositoryImpl::new(pool);
                let mut update_company_service = UpdateCompanyService::new(Box::new(stocks_repository));
                let _result = update_company_service.update_company_info();
                println!("Company info updated");
            },
            &_ => {}
        }
    }
}
