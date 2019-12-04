CREATE DATABASE grpc OWNER you;
\c grpc;

CREATE TABLE users(
  id VARCHAR(255) PRIMARY KEY,
  first_name VARCHAR(255) NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  date_of_birth Date NOT NULL
);

INSERT INTO users VALUES
    ('steadylearner', 'steady', 'learner', '2019-01-01');
INSERT INTO users VALUES
    ('mybirthdayisblackfriday', 'mybirthdayis', 'blackfriday', '2019-11-25');
INSERT INTO users VALUES
    ('mybirthdayisnotblackfriday', 'mybirthdayis', 'notblackfriday', '2019-11-26');

-- CREATE TABLE users(
--   id VARCHAR(255) PRIMARY KEY,
--   first_name VARCHAR(255) NOT NULL,
--   last_name VARCHAR(255) NOT NULL,
--   date_of_birth BIGINT NOT NULL
-- );

-- INSERT INTO users VALUES
--     ('steadylearner', 'steady', 'learner', '1546300800000');
-- INSERT INTO users VALUES
--     ('mybirthdayisblackfriday', 'mybirthdayis', 'blackfriday', '1574640000000');
-- INSERT INTO users VALUES
--     ('mybirthdayisnotblackfriday', 'mybirthdayis', 'notblackfriday', '1574726400000');
