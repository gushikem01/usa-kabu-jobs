use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct StocksDTO {
    pub symbol: String,
    pub name : Option<String>,
    pub price: Option<f64>,
    pub exchange: Option<String>,
    pub exchangeShortName: Option<String>,
    pub r#type: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CompanyProfileDTO {
    pub symbol: String,
    pub companyName: String,
    pub price: f64,
}
