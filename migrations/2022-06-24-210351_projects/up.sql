-- Your SQL goes here

CREATE TABLE Projects(
    id SERIAL PRIMARY KEY,
    title TEXT NULL,
    body TEXT NULL,
    gallery_name TEXT NULL,
    name_tech TEXT []
);

CREATE TABLE Skills(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    name_tech TEXT NOT NULL,
    expe TEXT NOT NULL
);