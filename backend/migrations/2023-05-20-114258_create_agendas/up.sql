-- Your SQL goes here
CREATE TABLE agendas (
   id serial PRIMARY KEY,
   title VARCHAR (50) NOT NULL,
   deadline TIMESTAMP NOT NULL
);