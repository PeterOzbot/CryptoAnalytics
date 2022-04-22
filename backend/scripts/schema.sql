-- database
DROP DATABASE crypto_analytics;
CREATE DATABASE crypto_analytics;
\c crypto_analytics;

-- definitions
CREATE TABLE definitions (
    api_key text NOT NULL PRIMARY KEY,
    precision SMALLINT
);

-- entries
CREATE TABLE entries(
    id uuid NOT NULL PRIMARY KEY,
    definition_id text NOT NULL REFERENCES definitions (api_key),
    date_time timestamp NOT NULL,
    amount NUMERIC(19, 12),
    withdraw_fee NUMERIC(19, 12),
    price NUMERIC(19, 12),
    purchase_fee NUMERIC(19, 12)
);
