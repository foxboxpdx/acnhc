// Define structs that describe database table entities
use super::schema::*;

// Items - a more-or-less 'static' table containing data about the various kinds of 
// collectable items in the game.
#[derive(Identifiable, Queryable, AsChangeset, Serialize)]
pub struct Item {
    pub id: i32,
    pub name_en: String,
    pub name_jp: String,
    pub pri_type: i32,
    pub sub_type: i32
}

// OwnedItems - The big table.  Contains one entry for each of a given user's
// collected items.
#[derive(Identifiable, Queryable, AsChangeset, Serialize)]
pub struct Owneditem {
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
    pub extra: i32
}

// Users - Pretty self explanitory
#[derive(Identifiable, Queryable, AsChangeset, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub passwd: String,
    pub seen: String
}

// PrimaryTypes - Define the primary 'type' of a given Item, ie fossil,
// recipe, fish, bug, etc.
#[derive(Identifiable, Queryable, AsChangeset, Serialize)]
pub struct Primarytype {
    pub id: i32,
    pub name: String
}

// SubTypes - Define any sort of 'subtype' that may exist for a primary type,
// ie for recipes, what kind of recipe
#[derive(Identifiable, Queryable, AsChangeset, Serialize)]
pub struct Subtype {
    pub id: i32,
    pub name: String
}

// Now define 'insertable' versions of the structs for adding new data to
// each table

#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub name_en: &'a str,
    pub name_jp: &'a str,
    pub pri_type: i32,
    pub sub_type: i32
}

#[derive(Insertable)]
#[table_name="owneditems"]
pub struct NewOwneditem {
    pub user_id: i32,
    pub item_id: i32,
    pub extra: i32
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub passwd: &'a str
}

#[derive(Insertable)]
#[table_name="primarytypes"]
pub struct NewPrimarytype<'a> {
    pub name: &'a str
}

#[derive(Insertable)]
#[table_name="subtypes"]
pub struct NewSubtype<'a> {
    pub name: &'a str 
}

