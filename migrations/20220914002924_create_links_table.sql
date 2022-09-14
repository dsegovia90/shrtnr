-- Add migration script here
CREATE TABLE links(
  id TEXT NOT NULL,
  PRIMARY KEY (id),
  url TEXT NOT NULL,
  created_at timestamptz NOT NULL
);
