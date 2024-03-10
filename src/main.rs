mod application;
mod domain;
mod infrastructure;
mod schema;

use application::usecase::stocks_usecase::StocksService;
use infrastructure::database::establish_connection;
use infrastructure::adapters::stocks_repository::StocksRepositoryImpl;

fn main () {
    let pool = establish_connection();

    let stocks_repository = StocksRepositoryImpl::new(pool);

    let mut stocks_service = StocksService::new(Box::new(stocks_repository));

    let _result = stocks_service.create_stocks();

}
