#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
//#[macro_use] extern crate serde_derive;

extern crate acnhc_web;
extern crate acnhc_db;

use acnhc_web::*;
//use acnhc_db::models::*;
use acnhc_db::*;
//use std::collections::HashMap;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;
use rocket_contrib::databases::diesel;
use uuid::Uuid;

// Form and Context structs are stored in lib.rs

// Hello database
#[database("sqlite_db")]
struct Conn(diesel::SqliteConnection);

#[post("/dologin", data = "<userid>")]
fn dologin(mut cookies: Cookies, userid: Form<Userid>) -> Redirect {
    cookies.add(Cookie::new("userid", userid.into_inner().id));
    Redirect::to("/")
}

#[get("/login")]
fn login() -> Template {
    let context = EmptyContext{};
    Template::render("login", &context)
}

#[post("/newuser")]
fn newuser(conn: Conn, mut cookies: Cookies) -> Redirect {
    // Make a UUID for the new user
    let uuid = Uuid::new_v4();
    let uuidstr = uuid.to_hyphenated().to_string();
    let alias = "None".to_string();

    // Set it as a cookie for later
    cookies.add(Cookie::new("userid", uuidstr.to_string()));

    // Generate a NewUser with the UUID and put it in the database
    // diesel stuff
    let _ = acnhc_db::create_user(&*conn, &uuidstr, &alias);

    // Send user back to main
    Redirect::to("/")
}

#[get("/")]
fn index(conn: Conn, cookies: Cookies) -> Template {
    let cookie = cookies.get("userid");
    // If our cookie isn't set, redirect to login.
    // Otherwise get the user's data and counts from the join tables
    if let Some(ref cookie) = cookie {
        let uname = cookie.value();
        let uid = get_uid_from_uname(&*conn, &uname);
        // This returns 0 on error so uh...do something
        if uid == 0 {
            let x = EmptyContext {};
            Template::render("login", &x);
        }
        let fcount = count_owned_fossils(&*conn, uid);
        let rcount = count_owned_recipes(&*conn, uid);
        let context = IndexContext { user: uname.to_string(),
                                     fossils: fcount,
                                     recipes: rcount };
        // ok cool
        Template::render("index", &context)
    } else {
        let context = EmptyContext {};
        Template::render("login", &context)
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, dologin, login, newuser])
        .mount("/", StaticFiles::from("public"))
        .attach(Template::fairing())
        .attach(Conn::fairing())
}

fn main() {
    rocket().launch();
}
