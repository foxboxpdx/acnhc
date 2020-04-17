extern crate acnhc_db;
extern crate diesel;

use self::acnhc_db::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Fossil name?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nOk!\n");

    let _ = create_fossil(&connection, name);
    println!("\nSaved fossil {}", name);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
