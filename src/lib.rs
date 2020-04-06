// Define helper functions and structs for ACNHC
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
use rocket_contrib::databases::rusqlite;

pub mod bug;
pub mod clothing;
pub mod fish;
pub mod flower;
pub mod fossil;
pub mod furniture;
pub mod item;
pub mod recipe;

// Main totals page
#[derive(Serialize)]
pub struct CountContext {
    pub furncount: i64,
    pub clothcount: i64,
    pub fishcount: i64,
    pub bugcount: i64,
    pub fossilcount: i64,
    pub flowercount: i64,
    pub recipecount: i64,
    pub itemcount: i64
}

