CREATE TABLE todos (
   id serial PRIMARY KEY,
   title VARCHAR (50) NOT NULL,
   description TEXT NULL,
   visibility SMALLINT NOT NULL
);