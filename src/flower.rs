// Flowers!
#[derive(Serialize)]
pub struct Flower {
    pub id: i32,
    pub species: String,
    pub color: String,
    pub hybrid: bool,
    pub sellprice: i32
}

