// Models for dat data
use super::schema::*;

#[derive(Identifiable, Queryable, Associations, Serialize)]
pub struct Fossil {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="fossils"]
pub struct NewFossil<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations, AsChangeset, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub alias: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub alias: &'a str,
}

#[derive(Identifiable, Queryable, AsChangeset, Serialize)]
pub struct Ownedfossil {
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
    pub extra: i32
}

#[derive(Insertable)]
#[table_name="ownedfossils"]
pub struct NewOwnedfossil {
    pub user_id: i32,
    pub item_id: i32,
    pub extra: i32
}

#[derive(Identifiable, Queryable, Serialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name="recipes"]
pub struct NewRecipe<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, AsChangeset, Serialize)]
pub struct Ownedrecipe {
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
    pub extra: i32
}

#[derive(Insertable)]
#[table_name="ownedrecipes"]
pub struct NewOwnedrecipe {
    pub user_id: i32,
    pub item_id: i32,
    pub extra: i32
}

#[derive(Identifiable, Queryable, Serialize)]
pub struct Art {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[table_name="arts"]
pub struct NewArt<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, AsChangeset, Serialize)]
pub struct Ownedart {
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
    pub extra: i32
}

#[derive(Insertable)]
#[table_name="ownedarts"]
pub struct NewOwnedart {
    pub user_id: i32,
    pub item_id: i32,
    pub extra: i32
}
