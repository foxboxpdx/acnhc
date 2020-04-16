// Mod types
// Define a bunch of structs that correlate to tables in the database

// Bugs!
#[derive(Serialize)]
pub struct Bug {
    pub id: i32,
    pub name: String,
    pub modifiers: String,
    pub special: String,
    pub months: String,
    pub hours: String,
    pub sellprice: i32
}

// Clothing
#[derive(Serialize)]
pub struct Clothing {
    pub id: i32,
    pub name: String,
    pub ctype: String,
    pub sellprice: String,
    pub buyprice: String,
    pub variants: Vec<String>
}

// Clothing Variants
pub struct ClothingVariants {
    pub id: i32,
    pub name: String,
    pub clothing_id: i32
}


// Fish!
#[derive(Serialize)]
pub struct Fish {
    pub id: i32,
    pub name: String,
    pub modifiers: String,
    pub months: String,
    pub hours: String,
    pub sellprice: i32
}

// Flowers!
#[derive(Serialize)]
pub struct Flower {
    pub id: i32,
    pub species: String,
    pub color: String,
    pub hybrid: bool,
    pub sellprice: i32
}

// Fossils!
#[derive(Serialize)]
pub struct Fossil {
    pub id: i32,
    pub name: String,
    pub sellprice: i32
}

// Furniture
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

// Furniture Variants
pub struct FurnitureVariant {
    pub id: i32,
    pub name: String,
    pub furniture_id: i32
}

// Items!
#[derive(Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub sellprice: i32
}

// Recipes!
#[derive(Serialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub rtype: String
}

// Walls n floors!
#[derive(Serialize)]
pub struct Wallfloor {
    pub id: i32,
    pub name: String,
    pub wall: bool,
    pub size: i32,
    pub animated: bool,
    pub sellprice: i32,
    pub buyprice: i32
}
// Consider collapsing wall/size/animated into a bitmap
