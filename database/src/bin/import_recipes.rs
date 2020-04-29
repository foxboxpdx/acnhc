extern crate acnhc_db;
extern crate diesel;

use diesel::prelude::*;
use self::acnhc_db::*;
use self::acnhc_db::models::*;
use self::acnhc_db::schema::recipes;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let category: i32 = args[2].parse().unwrap();

    // Read the file
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read file");

    let mut recipes: Vec<&str> = contents.split("\n").collect();
    let _ = recipes.pop();

    // Connect to DB and dump the contents in
    let connection = establish_connection();

    for name in recipes.iter() {
        let new_r = NewRecipe { name: name, type_id: category };
        let _ = diesel::insert_into(recipes::table).values(&new_r)
                .execute(&connection)
                .expect("error saving new recipe");
        println!("\nSaved recipe {}", name);
    }
}

