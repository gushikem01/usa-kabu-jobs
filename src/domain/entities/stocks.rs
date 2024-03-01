use crate::schema::stocks;
use diesel::Insertable;

#[derive(Debug, Clone)]
pub struct Stocks {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub exchange_short_name: String,
    pub is_etf: bool
}

#[derive(Debug, Insertable)]
#[table_name="stocks"]
pub struct NewStocks {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub exchange_short_name: String,
    pub is_etf: bool
}

// impl Stocks {
//     pub fn new(symbol: String,
//         name: String,
//         exchange: String,
//         exchange_short_name: String,
//         is_etf: bool) -> NewStocks {
//             NewStocks {
//             symbol,
//             name,
//             exchange,
//             exchange_short_name,
//             is_etf
//         }
//     }
// }
