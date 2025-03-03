-- Your SQL goes here
CREATE TABLE martial_arts (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    image_url TEXT,
    country_of_origin TEXT NOT NULL,
    monthly_price_usd REAL NOT NULL
);
