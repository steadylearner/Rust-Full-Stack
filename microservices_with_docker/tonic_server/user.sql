-- http://www.postgresqltutorial.com/postgresql-uuid/
-- CREATE DATABASE grpc OWNER you;
-- \c grpc;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- SELECT uuid_generate_v4();

CREATE TABLE users (
    -- use VARCHAR instead of uuid because of the type problem.
    -- Should find the better way and modify the code later if necessary.
    -- It is also easy to test end points with this.
    id VARCHAR(255) PRIMARY KEY DEFAULT uuid_generate_v4 (),
    first_name VARCHAR(60) NOT NULL,
    last_name VARCHAR(60) NOT NULL,
    date_of_birth Date NOT NULL
);

INSERT INTO users
    (id, first_name, last_name, date_of_birth)
VALUES
    ('steadylearner', 'steady', 'learner', '2019-01-01');

INSERT INTO users
    (id, first_name, last_name, date_of_birth)
VALUES
    ('mybirthdayisblackfriday', 'mybirthdayis', 'blackfriday', '2019-11-25');

INSERT INTO users
    (id, first_name, last_name, date_of_birth)
VALUES
    ('mybirthdayisnotblackfriday', 'mybirthdayis', 'notblackfriday', '2019-11-26');

