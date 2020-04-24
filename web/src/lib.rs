// Define helper functions and structs for ACNHC
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate acnhc_db;

use acnhc_db::models::*;

// Form structs //

// Login struct
#[derive(FromForm)]
pub struct Userid {
    pub id: String,
}

// Alias setting struct
#[derive(FromForm)]
pub struct Alias {
    pub id: String,
    pub alias: String,
}

// Edit form struct
// XJ and OJ are json-encoded hashmaps
#[derive(FromForm, Debug)]
pub struct EditForm {
    pub owned: String,
    pub extra: String,
    pub oj: String,
    pub xj: String
}

// Context structs //

// Empty context for testing
#[derive(Serialize)]
pub struct EmptyContext {}

// Basic struct just containing a User
#[derive(Serialize)]
pub struct UserContext {
  pub user: User
}

// Index/main context
#[derive(Serialize)]
pub struct IndexContext {
    pub user: String,
    pub fossils: i64,
    pub recipes: i64,
}

// Single user fossil data
#[derive(Serialize)]
pub struct FossilSelfContext {
    pub user: User,
    pub fossils: Vec<Fossil>,
    pub owned: Vec<Ownedfossil>,
}

// All user fossil data
#[derive(Serialize)]
pub struct FossilAllContext {
    pub users: Vec<User>,
    pub owned: Vec<Ownedfossil>,
    pub fossils: Vec<Fossil>,
}

// Single user recipe data
#[derive(Serialize)]
pub struct RecipeSelfContext {
    pub user: User,
    pub recipes: Vec<Recipe>,
    pub owned: Vec<Ownedrecipe>
}

// All user recipe data
#[derive(Serialize)]
pub struct RecipeAllContext {
    pub users: Vec<User>,
    pub owned: Vec<Ownedrecipe>,
    pub recipes: Vec<Recipe>
}
