-- Add up migration script here
-- Up
CREATE TABLE quotes (
    id TEXT PRIMARY KEY,
    text TEXT NOT NULL,
    author TEXT NOT NULL,
    creator TEXT NOT NULL
);