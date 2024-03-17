// @generated automatically by Diesel CLI.

diesel::table! {
    schema_migrations (version) {
        version -> Int8,
        inserted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    stocks (id) {
        id -> Uuid,
        #[max_length = 255]
        symbol -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        price -> Numeric,
        market_cap -> Numeric,
        last_dividend -> Numeric,
        #[max_length = 255]
        exchange -> Varchar,
        #[max_length = 255]
        exchange_short_name -> Varchar,
        #[max_length = 255]
        website -> Varchar,
        description -> Text,
        ipo_date -> Nullable<Date>,
        is_etf -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    schema_migrations,
    stocks,
);
