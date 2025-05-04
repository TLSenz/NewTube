-- Your SQL goes here
CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       username TEXT not null ,
                       email TEXT not null ,
                       password TEXT not null
)
