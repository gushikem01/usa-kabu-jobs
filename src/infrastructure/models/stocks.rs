use crate::schema::stocks;
use diesel::{Insertable, Queryable, Selectable};
use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Debug, Insertable)]
#[diesel(table_name = stocks)]
pub struct NewStocks {
    pub symbol: String,
    pub name: Option<String>,
    pub price: Option<BigDecimal>,
    pub exchange: String,
    pub exchange_short_name: String,
    pub is_etf: bool,
}

#[derive(Debug, Queryable, Selectable, QueryableByName)]
#[diesel(table_name = stocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Stocks {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub price: BigDecimal,
    pub exchange: String,
    pub exchange_short_name: String,
    pub is_etf: bool,
}
