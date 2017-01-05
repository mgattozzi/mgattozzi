CREATE TABLE counts (
  id SERIAL PRIMARY KEY,
  clicks SERIAL NOT NULL
);

INSERT INTO counts (clicks) VALUES (0);
