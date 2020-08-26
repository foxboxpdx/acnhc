// Quick CLI program to import various pieces of data into the database
extern crate acnhc_rest;
extern crate diesel;

use self::acnhc_rest::*;
use self::acnhc_rest::models::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filetype = &args[1];
    let filename = &args[2];

    // Branch based on the filetype
    match filetype.as_str() {
        "item" => process_items(filename),
        "primary" => process_primary(filename),
        "sub" => process_sub(filename),
        "user" => process_user(filename),
        _ => println!("Unknown filetype")
    };
}

// Import a CSV of items into the database.  Requires primary and subtypes to be set as it will have
// to look those up
fn process_items(fname: &str) {}

// Import a CSV defining primary types into the database
fn process_primary(fname: &str) {}

// Import a CSV defining subtypes into the database
fn process_sub(fname: &str) {}

// Import a CSV of users int othe database
fn process_user(fname: &str) {}
