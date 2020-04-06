// Define helper functions and structs for ACNHC
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
use rocket_contrib::databases::rusqlite;

pub mod types;
use crate::types::*;

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

// Bugs page
#[derive(Serialize)]
pub struct BugContext {
    pub category: String,
    pub bugs: Vec<Bug>
}

// Clothing page
#[derive(Serialize)]
pub struct ClothingContext {
    pub category: String,
    pub clothing: Vec<Clothing>
}

// Fish page
#[derive(Serialize)]
pub struct FishContext {
    pub category: String,
    pub fish: Vec<Fish>
}

// Flowers page
#[derive(Serialize)]
pub struct FlowerContext {
    pub category: String,
    pub flowers: Vec<Flower>
}

// Fossils page
#[derive(Serialize)]
pub struct FossilContext {
    pub category: String,
    pub fossils: Vec<Fossil>
}

// Furniture page
#[derive(Serialize)]
pub struct FurnitureContext {
    pub category: String,
    pub furniture: Vec<Furniture>
}

// Items page
#[derive(Serialize)]
pub struct ItemContext {
    pub category: String,
    pub items: Vec<Item>
}

// Recipes page
#[derive(Serialize)]
pub struct RecipeContext {
    pub category: String,
    pub recipes: Vec<Recipe>
}

// Walls n floors page
#[derive(Serialize)]
pub struct WallfloorContext {
    pub category: String,
    pub wallfoors: Vec<Wallfloor>
}

