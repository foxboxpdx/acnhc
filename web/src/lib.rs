// Define helper functions and structs for ACNHC
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate acnhc_db;

use std::collections::HashMap;
use rocket::http::RawStr;
use acnhc_db::models::*;
// Form structs

// Raw form data
#[derive(FromForm)]
pub struct RawForm<'f> {
    pub value: &'f RawStr
}

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

// Fossil form struct
#[derive(FromForm)]
pub struct FossilForm {
    pub id: String,
    pub owned: String,
    pub extra: String,
}

// Recipe form struct
#[derive(FromForm)]
pub struct RecipeForm {
    pub id: String,
    pub owned: String,
    pub extra: String,
}

// Context structs

// Empty context for testing
#[derive(Serialize)]
pub struct EmptyContext {}

// Index/main context
#[derive(Serialize)]
pub struct IndexContext {
    pub user: String,
    pub fossils: i32,
    pub recipes: i32,
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
