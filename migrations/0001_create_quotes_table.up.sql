-- Add up migration script here
CREATE TABLE IF NOT EXISTS quotes (
    id VARCHAR(200) UNIQUE PRIMARY KEY NOT NULL,
    qtext VARCHAR(200) NOT NULL,
    author VARCHAR(200) NOT NULL,
    creator VARCHAR(200) NOT NULL
);