use serde::Deserialize;
use crate::domain::entities::company_profile::CompanyProfile;
use crate::utils::strings::f64_to_bigdecimal;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct CompanyProfileDTO {
    pub symbol: String,
    pub companyName: String,
    pub mktCap: f64,
    pub price: f64,
}

impl CompanyProfileDTO {
    pub fn to_domain(&self) -> CompanyProfile {
        CompanyProfile {
            symbol : self.symbol.clone(),
            price: f64_to_bigdecimal(self.price),
            market_cap: f64_to_bigdecimal(self.mktCap),
        }
    }
}
