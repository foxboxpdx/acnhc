#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate acnhc_web;

use acnhc_web::*;
use std::collections::HashMap;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::templates::Template;
use uuid::Uuid;

// Form and Context structs are stored in lib.rs

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
fn newuser(mut cookies: Cookies) -> Redirect {
    // Make a UUID for the new user
    let uuid = Uuid::new_v4();
    let uuidstr = uuid.to_hyphenated().to_string();

    // Set it as a cookie for later
    cookies.add(Cookie::new("userid", uuidstr));

    // Generate a NewUser with the UUID and put it in the database
    // diesel stuff

    // Send user back to main
    Redirect::to("/")
}

#[get("/")]
fn index(cookies: Cookies) -> Template {
    let cookie = cookies.get("userid");
    let mut context = HashMap::new();
    if let Some(ref cookie) = cookie {
        context.insert("userid", cookie.value());
        Template::render("index", &context)
    } else {
        Template::render("login", &context)
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, dologin, login, newuser])
        .mount("/", StaticFiles::from("public"))
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
