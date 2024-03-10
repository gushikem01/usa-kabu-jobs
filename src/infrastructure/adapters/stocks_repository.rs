use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use crate::domain::entities::stocks::Stocks;
use crate::infrastructure::models::stocks::NewStocks;
use crate::domain::repositories::stocks_repository::StocksRepository;
use crate::schema::stocks;

const CHUNK_SIZE: i32 = 10000;

pub struct StocksRepositoryImpl {
    pub pg_conn: PgConnection,
}

impl StocksRepositoryImpl {
    pub fn new(pg_conn: PgConnection) -> StocksRepositoryImpl {
        StocksRepositoryImpl {
            pg_conn: pg_conn,
        }
    }
}

impl StocksRepository for StocksRepositoryImpl {
    fn create_stocks(&mut self, stocks: Vec<Stocks>) -> Result<(), String> {
        let chunks = stocks.chunks(CHUNK_SIZE as usize);

        for chunk in chunks {
            let new_stocks: Vec<NewStocks> = chunk.into_iter().map(|stock| {
                NewStocks {
                    symbol: stock.symbol.clone(),
                    name: Some(stock.name.clone()),
                    exchange: stock.exchange.clone(),
                    exchange_short_name: stock.exchange_short_name.clone(),
                    is_etf: stock.is_etf,
                }
            }).collect();

            let res = self.pg_conn.transaction::<_, diesel::result::Error, _>(|conn| {

                let res = diesel::insert_into(stocks::table)
                    .values(&new_stocks)
                    .execute(conn)?;

                match res {
                    0 => Err(diesel::result::Error::NotFound),
                    _ => Ok(()),
                }
            });

            match res {
                Ok(_) => continue,
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(())
    }
}
