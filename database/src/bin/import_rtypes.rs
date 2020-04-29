extern crate acnhc_db;
extern crate diesel;

use self::acnhc_db::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Read the file
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read file");

    let mut recipetypes: Vec<&str> = contents.split("\n").collect();
    let _ = recipetypes.pop();

    // Connect to DB and dump the contents in
    let connection = establish_connection();

    for name in recipetypes.iter() {
        let _ = create_recipetype(&connection, name);
        println!("\nSaved recipetype {}", name);
    }
}

