CREATE TABLE token_key (
    id SERIAL UNIQUE PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    token bytea NOT NULL,
    expiration TIMESTAMP NOT NULL
);