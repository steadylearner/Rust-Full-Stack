CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  stock FLOAT NOT NULL,
  price INTEGER --representing cents
);
