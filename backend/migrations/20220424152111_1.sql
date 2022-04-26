-- definitions
CREATE TABLE definitions (
    api_key text NOT NULL PRIMARY KEY,
    precision SMALLINT NOT NULL 
);

-- entries
CREATE TABLE entries(
    id uuid NOT NULL PRIMARY KEY,
    definition_id text NOT NULL REFERENCES definitions (api_key),
    date_time timestamp NOT NULL,
    amount NUMERIC(19, 12) NOT NULL,
    withdraw_fee NUMERIC(19, 12) NOT NULL,
    price NUMERIC(19, 12) NOT NULL,
    purchase_fee NUMERIC(19, 12) NOT NULL
);
