use crate::schema::stocks;
use diesel::Insertable;
use bigdecimal::BigDecimal;

#[derive(Debug, Insertable)]
#[diesel(table_name = stocks)]
pub struct NewStocks {
    pub symbol: String,
    pub name: Option<String>,
    pub price: Option<BigDecimal>,
    pub exchange: String,
    pub exchange_short_name: String,
    pub is_etf: bool
}
