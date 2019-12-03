-- http://www.postgresqltutorial.com/postgresql-uuid/
-- CREATE DATABASE auth OWNER you;
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- SELECT uuid_generate_v4();

-- Refer to www.twitter.com, www.github.com
-- linekedin url
-- github url

-- CREATE DOMAIN email AS TEXT CHECK (VALUE ~* '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$');
-- ERROR:  unique constraints not possible for domains

CREATE TABLE users (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
  username VARCHAR(60) UNIQUE NOT NULL,
  first_name VARCHAR(60),
  last_name VARCHAR(60),
  email VARCHAR(255) UNIQUE NOT NULL
  CONSTRAINT valid_email CHECK (email ~* '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
  password VARCHAR NOT NULL CHECK (char_length(password) >= 5),

  -- Temporary solution because session in redis can not block other loged in users
  -- to visit other accounts currently
  identity_id uuid UNIQUE NOT NULL DEFAULT uuid_generate_v4 (),

  profile_description TEXT CHECK (char_length(profile_description) <= 160),
  website VARCHAR(2083) UNIQUE
  CONSTRAINT valid_website CHECK (website ~* 'https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,255}\.[a-z]{2,9}\y([-a-zA-Z0-9@:%_\+.~#?&//=]*)$'),
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO
users(id, username, first_name, last_name, email, password, profile_description, website)
VALUES(uuid_generate_v4(), 'Steadylearner', 'steady', 'learner', 'steady@learner.com', '$2b$10$GloY0CZo11sI89cT1sJEj.ET2oanFoelk5Tyhww6v7bIbAfACepVm', 'This is to test login project.', 'https://www.steadylearner.com');

INSERT INTO
users(id, username, first_name, last_name, email, password, profile_description, website)
VALUES(uuid_generate_v4(), 'John Doe', 'John', 'Doe', 'John@Doe.com', '$2b$10$GloY0CZo11sI89cT1sJEj.ET2oanFoelk5Tyhww6v7bIbAfACepVm', 'This is to test login project.', 'https://www.johndoe.com');