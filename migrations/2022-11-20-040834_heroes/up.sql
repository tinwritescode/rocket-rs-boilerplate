-- Your SQL goes here
CREATE TABLE heroes (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    fantasy_name VARCHAR NOT NULL,
    real_name VARCHAR NULL,
    spotted_photo TEXT NOT NULL,
    strength_level INT NOT NULL DEFAULT 0
);
