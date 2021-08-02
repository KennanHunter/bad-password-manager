-- Your SQL goes here
CREATE TABLE Users(
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email TEXT NOT NULL,
    pass VARCHAR(255) NOT NULL
)