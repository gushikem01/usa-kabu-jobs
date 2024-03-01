use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use crate::application::dtos::stocks_dto::StocksDTO;
use crate::domain::entities::stocks::NewStocks;
use crate::domain::repositories::stocks_repository::StocksRepository;
use crate::schema::stocks;

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
    fn create_stocks(&mut self, dtos: StocksDTO) -> Result<(), String> {

        let new_stocks = NewStocks {
            symbol: dtos.symbol.clone(),
            name: dtos.name.clone(),
            exchange: dtos.exchange.clone(),
            exchange_short_name: dtos.exchange_short_name.clone(),
            is_etf: dtos.is_etf,
        };

        let _ = self.pg_conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let _res = diesel::insert_into(stocks::table)
                .values(&new_stocks)
                .execute(conn)?;

                println!("{:?}", _res);
            Ok(())

        });

        Ok(())

    }
}
