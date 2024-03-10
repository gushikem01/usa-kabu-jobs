-- Your SQL goes here
CREATE TABLE stocks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol varchar(255) NOT NULL DEFAULT '',
    name varchar(255) NOT NULL DEFAULT '',
    exchange varchar(255) NOT NULL DEFAULT '',
    exchange_short_name varchar(255) NOT NULL DEFAULT '',
    is_etf BOOLEAN NOT NULL DEFAULT FALSE
);
