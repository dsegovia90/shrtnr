-- Add migration script here
CREATE TABLE links(
  id TEXT NOT NULL PRIMARY KEY,
  url TEXT NOT NULL,
  created_at timestamptz NOT NULL
);

-- Add migration script here
CREATE TABLE link_hits(
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  headers json NOT NULL,
  method TEXT NOT NULL,
  uri TEXT NOT NULL,
  version TEXT NOT NULL,
  peer_addr TEXT,
  connection_type TEXT,
  created_at timestamptz NOT NULL,
  link_id TEXT NOT NULL REFERENCES links(id)
);
