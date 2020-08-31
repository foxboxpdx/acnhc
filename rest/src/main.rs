// Define the actual REST web interface using Rocket
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate acnhc_rest;

use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel;
use acnhc_rest::models::*;
use acnhc_rest::web::*;
use acnhc_rest::*;

// Database
#[database("sqlite_db")]
struct Conn(diesel::SqliteConnection);

// Define paths
// 1: Logins
#[post("/authentorize", format = "application/json", data = "<user>")]
fn dologin(user: Json<UserRequest>) -> Json<UserResponse> {
    Json(UserResponse { username: "foo".to_string(), 
                        userid: 0, 
                        token: "foo".to_string()})
}

// 1b: New User
#[post("/cytokinesis", format = "application/json", data = "<user>")]
fn newuser(user: Json<UserRequest>) -> Json<UserResponse> {
    Json(UserResponse { username: "foo".to_string(), 
                        userid: 0, 
                        token: "foo".to_string()})
}

// 2: Data retrieval
#[post("/exhume", format = "application/json", data = "<data>")]
fn getdata(data: Json<DataRequest>) -> Json<DataResponse> {
    Json(DataResponse { items: Vec::new(), owned: Vec::new() })
}

// 3: Data saving/updating
#[post("/cognize", format = "application/json", data = "<data>")]
fn savedata(data: Json<UpdateRequest>) -> Json<UpdateResponse> {
    Json(UpdateResponse { ownedid: 0, extras: 0 })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![dologin, getdata, savedata])
        .attach(Conn::fairing())
}

fn main() {
    rocket().launch();
}
