extern crate acnhc_db;
extern crate diesel;

use self::acnhc_db::*;
use self::acnhc_db::models::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Read the file
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read file");

    let mut arts: Vec<&str> = contents.split("\n").collect();
    let _ = arts.pop();

    // Connect to DB and dump the contents in
    let connection = establish_connection();

    for name in arts.iter() {
        let _ = Art::create(&connection, name);
        println!("\nSaved fossil {}", name);
    }
}

