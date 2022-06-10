-- Add migration script here
PRAGMA foreign_key=ON;

CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);

CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    author INTEGER NOT NULL,
    published BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY (author) REFERENCES users (id)
);
