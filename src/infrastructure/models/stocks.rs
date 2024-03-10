use crate::schema::stocks;
use diesel::Insertable;

#[derive(Debug, Insertable)]
#[diesel(table_name = stocks)]
pub struct NewStocks {
    pub symbol: String,
    pub name: Option<String>,
    pub exchange: String,
    pub exchange_short_name: String,
    pub is_etf: bool
}
