CREATE TABLE mangas (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  publishingCompany VARCHAR NOT NULL,
  pages INTEGER,
  evaluation INTEGER,
  cover VARCHAR,
  read VARCHAR, 
  writer VARCHAR,
  artist VARCHAR,
  createdAt TIMESTAMP,
  updatedAt TIMESTAMP
)