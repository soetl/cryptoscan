CREATE TABLE IF NOT EXISTS coins
(
    id                  INTEGER PRIMARY KEY NOT NULL,
    name                TEXT NOT NULL,
    symbol              TEXT NOT NULL,
    price               REAL,
    volume_24h          REAL,
    percent_change_1h   REAL,
    percent_change_24h  REAL,
    percent_change_7d   REAL,
    market_cap          REAL, 
    last_updated        TEXT NOT NULL
);