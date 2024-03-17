#[macro_use]
extern crate diesel;

mod application;
mod domain;
mod infrastructure;
mod schema;
mod utils;

use application::usecase::stocks_usecase::StocksService;
use infrastructure::database::establish_connection;
use infrastructure::adapters::stocks_repository::StocksRepositoryImpl;
const ARGS_LENGTH: usize = 2;

fn main () {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < ARGS_LENGTH {
        println!("Please provide a command");
        return;
    }

    let pool = establish_connection();

    let stocks_repository = StocksRepositoryImpl::new(pool);

    let mut stocks_service = StocksService::new(Box::new(stocks_repository));

    for n in args {
        match n.as_str() {
            "create_stocks" => {
                let _result = stocks_service.create_stocks();
            },
            &_ => {
                println!("Command not found");
            }
        }
    }
}
