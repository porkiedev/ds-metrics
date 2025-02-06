CREATE TABLE IF NOT EXISTS users (
    user_id INT PRIMARY KEY,
    created_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS money_updates (
    id CHARACTER(26) PRIMARY KEY,
    user_id INT NOT NULL,
    amount INT NOT NULL,
    created_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS mile_updates (
    id CHARACTER(26) PRIMARY KEY,
    user_id INT NOT NULL,
    amount INT NOT NULL,
    created_at DATETIME NOT NULL
);
