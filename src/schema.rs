// @generated automatically by Diesel CLI.

diesel::table! {
    stocks (id) {
        id -> Uuid,
        #[max_length = 255]
        symbol -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        price -> Numeric,
        #[max_length = 255]
        exchange -> Varchar,
        #[max_length = 255]
        exchange_short_name -> Varchar,
        is_etf -> Bool,
    }
}
