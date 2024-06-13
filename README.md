# Rust Q and A server
Vienna Wagner 2024

Course project for Portland State University's Rust Web Development course, spring 2024 quarter. This will be a question and answer web server implemented in rust. It is an adaptation of the server from `Rust Web Development` by Bastian Gruber, adapted to use axum instead of warp.

## Usage
Runs on localhost:3000, use /questions for get, post, put, and delete requests, /answers also supports post requests to add answers.

## Database
I have yet to get docker working, but the project theoretically supports a postgresql backend if you set one up yourself. To create tables with the correct schema:
```
CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
```
and
```
CREATE TABLE IF NOT EXISTS answers (
   id serial PRIMARY KEY,
   content TEXT NOT NULL,
created_on TIMESTAMP NOT NULL DEFAULT NOW(),
   corresponding_question integer REFERENCES questions
);
```
By default, the program will attempt to connect to a database named `postgres` on `localhost:5432` as user `default` with password `1234`, but this can be modified by adjusting line 71 of `main.rs`