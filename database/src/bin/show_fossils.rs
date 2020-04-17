extern crate acnhc_db;
extern crate diesel;

use self::acnhc_db::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use acnhc_db::schema::fossils::dsl::*;

    let connection = establish_connection();
    let results = fossils
        .limit(5)
        .load::<Fossil>(&connection)
        .expect("Error loading fossils");

    println!("Displaying {} fossils", results.len());
    for fossil in results {
        println!("{}", fossil.name);
        println!("----------\n");
    }
}
