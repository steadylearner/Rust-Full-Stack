-- Your SQL goes here
CREATE TABLE invitations (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  expires_at TIMESTAMP NOT NULL
);
