CREATE TABLE callbacks (
  id SERIAL PRIMARY KEY,
  code TEXT NOT NULL,
  state VARCHAR(255) NOT NULL
);

CREATE TABLE tokens (
  id SERIAL PRIMARY KEY,
  access_token VARCHAR(300) NOT NULL,
  refresh_token VARCHAR(255) NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_id VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE credentials (
  id SERIAL PRIMARY KEY,
  user_id VARCHAR(255) NOT NULL UNIQUE,
  token_id INTEGER NOT NULL UNIQUE,
  FOREIGN KEY (user_id)
  REFERENCES users(user_id),
  FOREIGN KEY (token_id)
  REFERENCES tokens(id)
);
