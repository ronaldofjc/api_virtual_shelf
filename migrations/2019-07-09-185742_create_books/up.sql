CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  subTitle VARCHAR,
  publishingCompany VARCHAR NOT NULL,
  pages INTEGER,
  evaluation INTEGER,
  cover VARCHAR,
  read VARCHAR, 
  writer VARCHAR,
  createdAt TIMESTAMP,
  updatedAt TIMESTAMP
)