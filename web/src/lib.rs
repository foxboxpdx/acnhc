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

// Fossil Editing
#[derive(Serialize)]
pub struct FossilEditContext {
    pub user: User,
    pub fossils: Vec<Fossil>,
    pub owned: Vec<Ownedfossil>,
}

// Fossil Self Report
#[derive(Serialize)]
pub struct FossilSelfContext {
    pub id: String,
    pub fossils: Vec<Fossil>,
    pub owned: Vec<Ownedfossil>,
}

// Fossil All Report
#[derive(Serialize)]
pub struct FossilAllContext {
    pub users: Vec<User>,
    pub owned: Vec<Ownedfossil>,
    pub fossils: Vec<Fossil>,
}

// Fossil whogot report
#[derive(Serialize)]
pub struct FossilGotContext {
    pub users: Vec<User>,
    pub owned: Vec<Ownedfossil>,
    pub fossils: Vec<Fossil>,
}

// Fossil whoneed report
#[derive(Serialize)]
pub struct FossilNeedContext {
    pub users: Vec<User>,
    pub owned: Vec<Ownedfossil>,
    pub fossils: Vec<Fossil>,
}

// Recipe Editing
#[derive(Serialize)]
pub struct RecipeEditContext {
    pub id: String,
}

// Recipe Self Report
#[derive(Serialize)]
pub struct RecipeSelfContext {
    pub id: String,
}

// Recipe All Report
#[derive(Serialize)]
pub struct RecipeAllContext {
    pub id: String
}

// Recipe whogot report
#[derive(Serialize)]
pub struct RecipeGotContext {
    pub id: String,
}

// Recipe whoneed report
#[derive(Serialize)]
pub struct RecipeNeedContext {
    pub id: String,
}
