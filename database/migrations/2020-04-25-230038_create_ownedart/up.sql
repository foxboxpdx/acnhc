CREATE TABLE ownedarts (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  item_id INTEGER NOT NULL,
  extra INTEGER NOT NULL DEFAULT 0
)
