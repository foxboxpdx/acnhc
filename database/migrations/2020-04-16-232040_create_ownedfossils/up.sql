CREATE TABLE ownedfossils (
  id INTEGER PRIMARY KEY,
  user_id INTEGER NOT NULL,
  fossil_id INTEGER NOT NULL,
  owned BOOLEAN NOT NULL DEFAULT false,
  extra INTEGER NOT NULL DEFAULT 0
)
