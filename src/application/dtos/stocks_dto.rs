use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct StocksDTO {
    pub symbol: String,
    pub name : Option<String>,
    pub exchange: Option<String>,
    pub exchangeShortName: Option<String>,
    pub r#type: Option<String>,
}
