use clap::Parser;
use crate::application::usecase::create_stocks::StocksService;
use crate::application::usecase::update_company::UpdateCompanyService;
use crate::infrastructure::database::establish_connection;
use crate::infrastructure::adapters::stocks_repository::StocksRepositoryImpl;

#[derive(Debug, Parser)]
#[clap(name = "Stocks CLI", version = "0.1.0", author = "Stocks CLI", about = "Stocks CLI")]
pub struct Args {
    #[arg(default_value = "create_stocks")]
    pub name: Option<String>,
}

// args is a function that returns the parsed arguments
pub fn args() -> Args {
    Args::parse()
}

#[tokio::main]
pub async fn handler(args: Args) {
    let conn = establish_connection();
    let stocks_repository = StocksRepositoryImpl::new(conn);

    match args.name.as_deref() {
        Some("create_stocks") => {
            let mut stocks_service = StocksService::new(Box::new(stocks_repository));
            let _res = stocks_service.create_stocks().await;
        },
        Some("update_company_info") => {
            let mut update_company_service = UpdateCompanyService::new(Box::new(stocks_repository));
            let _res = update_company_service.update_company_info().await;
        },
        _ => {
            println!("Invalid command");
        }
    }
}
