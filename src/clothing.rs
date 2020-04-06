// Clothing: Describe a struct and helper functions for Clothing items and
// variations thereof.
#[derive(Serialize)]
pub struct Clothing {
    pub id: i32,
    pub name: String,
    pub ctype: String,
    pub sellprice: String,
    pub buyprice: String,
    pub variants: Vec<String>
}

// Variants struct
pub struct ClothingVariants {
    pub id: i32,
    pub name: String,
    pub clothing_id: i32
}


