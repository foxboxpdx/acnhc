CREATE TABLE IF NOT EXISTS "furnitures" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "length" FLOAT DEFAULT 1 NOT NULL, 
  "width" FLOAT DEFAULT 1 NOT NULL, 
  "floor" BOOLEAN DEFAULT 't' NOT NULL, 
  "wall" BOOLEAN DEFAULT 't' NOT NULL, 
  "sellprice" VARCHAR(12) DEFAULT 'Cannot Sell' NOT NULL, 
  "buyprice" VARCHAR(12) DEFAULT 'Cannot Buy' NOT NULL
);
CREATE UNIQUE INDEX "unique_furnitures_name" ON "furnitures" ("name");
CREATE TABLE IF NOT EXISTS "furniture_variants" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "furniture_id" INTEGER NOT NULL
);
CREATE INDEX "index_furniture_variants_furniture" ON "furniture_variants" ("furniture_id");
CREATE UNIQUE INDEX "unique_furniture_variants_key" ON "furniture_variants" ("id");
CREATE TABLE IF NOT EXISTS "wallfloors" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "wall" BOOLEAN DEFAULT 't' NOT NULL, 
  "size" INTEGER DEFAULT 0 NOT NULL, 
  "animated" BOOLEAN DEFAULT 'f' NOT NULL, 
  "sellprice" VARCHAR(12) DEFAULT 'Cannot Sell' NOT NULL, 
  "buyprice" VARCHAR(12) DEFAULT 'Cannot Buy' NOT NULL
);
CREATE UNIQUE INDEX "unique_wallfloors_name" ON "wallfloors" ("name");
CREATE TABLE IF NOT EXISTS "flowers" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "species" VARCHAR(256) NOT NULL, 
  "color" VARCHAR(256) NOT NULL, 
  "hybrid" BOOLEAN DEFAULT 'f' NOT NULL, 
  "sellprice" INTEGER DEFAULT 0 NOT NULL
);
CREATE TABLE IF NOT EXISTS "fish" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "river" BOOLEAN DEFAULT 'f' NOT NULL, 
  "sea" BOOLEAN DEFAULT 'f' NOT NULL, 
  "rivermouth" BOOLEAN DEFAULT 'f' NOT NULL, 
  "elevation" BOOLEAN DEFAULT 'f' NOT NULL, 
  "rain" BOOLEAN DEFAULT 'f' NOT NULL, 
  "months" VARCHAR(256) DEFAULT 'Year Round' NOT NULL, 
  "hours" VARCHAR(256) DEFAULT 'All day' NOT NULL, 
  "sellprice" INTEGER DEFAULT 0 NOT NULL
);
CREATE UNIQUE INDEX "unique_fish_name" ON "fish" ("name");
CREATE TABLE IF NOT EXISTS "bugs" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "rain" BOOLEAN DEFAULT 'f' NOT NULL, 
  "stump" BOOLEAN DEFAULT 'f' NOT NULL, 
  "rock" BOOLEAN DEFAULT 'f' NOT NULL, 
  "tree" BOOLEAN DEFAULT 'f' NOT NULL, 
  "special" VARCHAR(256) DEFAULT 'None', 
  "months" VARCHAR(256) DEFAULT 'Year Round' NOT NULL, 
  "hours" VARCHAR(256) DEFAULT 'All day' NOT NULL, 
  "sellprice" INTEGER DEFAULT 0 NOT NULL
);
CREATE UNIQUE INDEX "unique_bugs_name" ON "bugs" ("name");
CREATE TABLE IF NOT EXISTS "fossils" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "sellprice" INTEGER DEFAULT 0 NOT NULL
);
CREATE UNIQUE INDEX "unique_fossils_name" ON "fossils" ("name");
CREATE TABLE IF NOT EXISTS "clothings" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "ctype" VARCHAR(50) DEFAULT 'Hat' NOT NULL, 
  "sellprice" VARCHAR(12) DEFAULT 'Cannot Sell' NOT NULL, 
  "buyprice" VARCHAR(12) DEFAULT 'Cannot Buy' NOT NULL
);
CREATE UNIQUE INDEX "unique_clothings_name" ON "clothings" ("name");
CREATE TABLE IF NOT EXISTS "clothing_variants" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "clothing_id" INTEGER NOT NULL
);
CREATE INDEX "index_clothing_variants_clothing" ON "clothing_variants" ("clothing_id");
CREATE UNIQUE INDEX "unique_clothing_variants_key" ON "clothing_variants" ("id");
CREATE TABLE IF NOT EXISTS "recipes" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "rtype" VARCHAR(50) DEFAULT 'Tool' NOT NULL
);
CREATE UNIQUE INDEX "unique_recipes_name" ON "recipes" ("name");
CREATE TABLE IF NOT EXISTS "items" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
  "name" VARCHAR(256) NOT NULL, 
  "sellprice" INTEGER DEFAULT 0 NOT NULL
);
CREATE UNIQUE INDEX "unique_items_name" ON "items" ("name");

