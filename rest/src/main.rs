// Define the actual REST web interface using Rocket
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::{Json, Value};

fn main() { 
    rocket::ignite()
        .launch();
}
