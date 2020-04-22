#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
//#[macro_use] extern crate serde_derive;

extern crate acnhc_web;
extern crate acnhc_db;

use acnhc_web::*;
use acnhc_db::models::*;
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


// Root path routes
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
    let user = jamie_please(&conn, &cookies);
    // If user.id is 0 assume it's because the cookie is unset or we were
    // unable to find that user in the database.  And hope it's not a DB
    // error because lmao
    if user.id == 0 {
        let x = EmptyContext {};
        Template::render("login", &x);
    }
    let fcount = count_owned_fossils(&*conn, user.id);
    let rcount = count_owned_recipes(&*conn, user.id);
    let context = IndexContext { user: user.username.to_string(),
                                 fossils: fcount,
                                 recipes: rcount };
    // Should be good to render the status/index page
    Template::render("index", &context)
}

#[get("/alias")]
fn alias(conn: Conn, cookies: Cookies) -> Template {
    let user = jamie_please(&conn, &cookies);
    let context = UserContext { user: user };
    Template::render("alias", &context)
}

#[post("/setalias", data = "<alias>")]
fn setalias(conn: Conn, alias: Form<Alias>) -> Redirect {
    set_user_alias(&*conn, alias.id, &alias.alias);
    Redirect::to("/")
}

// Fossil routes
#[get("/edit")]
fn fedit() {}

#[get("/report")]
fn freport() {}

#[get("/allreport")]
fn fall() {}

#[get("/whogot")]
fn fwhogot() {}

#[get("/whoneed")]
fn fwhoneed() {}

// Recipe routes
#[get("/edit")]
fn redit() {}

#[get("/report")]
fn rreport() {}

#[get("/allreport")]
fn rall() {}

#[get("/whogot")]
fn rwhogot() {}

#[get("/whoneed")]
fn rwhoneed() {}

// Routes for saving form data
#[post("/fossil")]
fn fsave() {}

#[post("/recipe")]
fn rsave() {}

// LAUNCH DAT THING
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, dologin, login, newuser, alias, setalias])
        .mount("/", StaticFiles::from("public"))
        .mount("/fossil", routes![fedit, freport, fall, fwhogot, fwhoneed])
        .mount("/recipe", routes![redit, rreport, rall, rwhogot, rwhoneed])
        .mount("/save", routes![fsave, rsave])
        .attach(Template::fairing())
        .attach(Conn::fairing())
}

fn main() {
    rocket().launch();
}

/*
// Taken from the rocket examples - init db
embed_migrations!();
fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = Conn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}
*/

// Helper function to PUT THAT COOKIE DOOOOOWN
// Pulls username out of cookie and retrieves the User struct from the DB
fn jamie_please(conn: &Conn, cookies: &Cookies) -> User {
    let cookie = cookies.get("userid");
    if let Some(ref cookie) = cookie {
        let uname = cookie.value().to_string();
        get_user_from_uname(&*conn, &uname)
    } else {
        User { id: 0, username: "0".to_string(), alias: "0".to_string() }
    }
}
