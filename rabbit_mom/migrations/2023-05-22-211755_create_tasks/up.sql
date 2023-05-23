-- Your SQL goes here
CREATE TABLE tasks (
   id serial PRIMARY KEY,
   task_started BOOLEAN NOT NULL DEFAULT FALSE,
   task_finished BOOLEAN NOT NULL DEFAULT FALSE
);