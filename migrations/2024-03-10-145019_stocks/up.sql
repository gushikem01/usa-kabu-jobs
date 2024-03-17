-- Your SQL goes here
CREATE TABLE stocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol varchar(255) NOT NULL DEFAULT '',
    name varchar(255) NOT NULL DEFAULT '',
    price decimal NOT NULL DEFAULT 0,
    market_cap decimal NOT NULL DEFAULT 0,
    last_dividend decimal NOT NULL DEFAULT 0,
    exchange varchar(255) NOT NULL DEFAULT '',
    exchange_short_name varchar(255) NOT NULL DEFAULT '',
    website varchar(255) NOT NULL DEFAULT '',
    description text NOT NULL DEFAULT '',
    ipo_date date NULL DEFAULT NULL,
    is_etf BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);
