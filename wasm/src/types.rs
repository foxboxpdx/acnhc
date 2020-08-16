// Define some structs temporarily til we can just pull them in from 
// acnhc-db
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Collectable {
    pub id: i32,
    pub name: String,
    pub image: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Sharable {
    pub id: i32,
    pub obtained: bool,
    pub extra: i32
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Creature {
    pub id: i32,
    pub caught: bool,
    pub donated: bool
}