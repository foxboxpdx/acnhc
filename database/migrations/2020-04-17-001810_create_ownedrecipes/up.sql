CREATE TABLE ownedrecipes (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  recipe_id INTEGER NOT NULL,
  owned BOOLEAN NOT NULL DEFAULT false,
  extra INTEGER NOT NULL DEFAULT 0
)