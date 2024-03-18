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
        avg_volume -> Numeric,
        changes_percentage -> Numeric,
        change -> Numeric,
        day_low -> Numeric,
        day_high -> Numeric,
        description -> Text,
        eps -> Numeric,
        #[max_length = 255]
        exchange -> Varchar,
        #[max_length = 255]
        exchange_short_name -> Varchar,
        ipo_date -> Nullable<Date>,
        is_etf -> Bool,
        last_dividend -> Numeric,
        market_cap -> Numeric,
        #[max_length = 255]
        name -> Varchar,
        open -> Numeric,
        previous_close -> Numeric,
        price -> Numeric,
        price_avg50 -> Numeric,
        price_avg200 -> Numeric,
        #[max_length = 255]
        symbol -> Varchar,
        volume -> Numeric,
        #[max_length = 255]
        website -> Varchar,
        year_high -> Numeric,
        year_low -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    schema_migrations,
    stocks,
);
