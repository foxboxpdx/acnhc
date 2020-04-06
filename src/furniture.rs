// Furniture: Describe a struct and helper functions for Furniture items and
// variations thereof.

#[derive(Serialize)]
pub struct Furniture {
    pub id: i32,
    pub name: String,
    pub length: f32,
    pub width: f32,
    pub floor: bool,
    pub wall: bool,
    pub sellprice: String,
    pub buyprice: String,
    pub variants: Vec<String>
}

// A struct for variants so we don't have to deal with the raw db data
pub struct FurnitureVariant {
    pub id: i32,
    pub name: String,
    pub furniture_id: i32
}

