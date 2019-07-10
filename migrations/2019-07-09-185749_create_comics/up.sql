CREATE TABLE comics (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  subTitle VARCHAR,
  serie VARCHAR,
  publishingCompany VARCHAR NOT NULL,
  pages INTEGER,
  evaluation INTEGER,
  cover VARCHAR,
  type VARCHAR,
  read VARCHAR, 
  writer VARCHAR,
  artist VARCHAR,
  createdAt TIMESTAMP,
  updatedAt TIMESTAMP
)