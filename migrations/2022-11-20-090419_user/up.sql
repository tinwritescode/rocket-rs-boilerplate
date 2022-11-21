-- Your SQL goes here
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    UNIQUE (email)
);

CREATE TABLE tokens (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INT NOT NULL,
    token VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    UNIQUE (user_id, token)
);