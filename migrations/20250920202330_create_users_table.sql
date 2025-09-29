-- Add migration script here
CREATE TABLE users (
      id SERIAL PRIMARY KEY,
      name VARCHAR(255) NOT NULL,
      phone VARCHAR(255) NOT NULL UNIQUE
);