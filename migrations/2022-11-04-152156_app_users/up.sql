-- Your SQL goes here
CREATE TABLE APP_USERS (
  id serial PRIMARY KEY,
  username varchar NOT NULL,
  password VARCHAR,
  active boolean,
  create_date timestamp)
