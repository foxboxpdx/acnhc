#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_fossil<'a>(conn: &SqliteConnection, name: &'a str) -> usize {
    use schema::fossils;

    let new_fossil = NewFossil {
        name: name,
    };

    diesel::insert_into(fossils::table)
        .values(&new_fossil)
        .execute(conn)
        .expect("Error saving new fossil")
}

pub fn create_user<'a>(conn: &SqliteConnection, n: &'a str, a: &'a str) -> usize {
    use schema::users;

    let new_user = NewUser {
        username: n,
        alias: a
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user")
}


