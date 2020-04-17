extern crate acnhc_db;
extern crate diesel;

use self::acnhc_db::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];

    // Read the file
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read file");

    let fossils: Vec<&str> = contents.split("\n").collect();

    // Connect to DB and dump the contents in
    let connection = establish_connection();

    for name in fossils.iter() {
        let _ = create_fossil(&connection, name);
        println!("\nSaved fossil {}", name);
    }
}

