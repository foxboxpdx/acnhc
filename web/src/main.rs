#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
//#[macro_use] extern crate serde_derive;

extern crate url;
extern crate acnhc_web;
extern crate acnhc_db;

use acnhc_web::*;
use acnhc_db::models::*;
use acnhc_db::*;
use std::collections::BTreeMap;
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

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::named("userid"));
    Redirect::to("/login")
}

#[get("/login")]
fn login() -> Template {
    let context = EmptyContext{};
    Template::render("login", &context)
}

#[get("/uuid?<uuidstr>")]
fn uuidlogin(mut cookies: Cookies, uuidstr: String) -> Redirect {
    cookies.add(Cookie::new("userid", uuidstr));
    Redirect::to("/")
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
    if user.id < 0 {
        let x = EmptyContext {};
        return Template::render("login", &x);
    }
    let f  = Ownedfossil::count(&*conn, user.id);
    let r  = Ownedrecipe::count(&*conn, user.id);
    let a  = Ownedart::count(&*conn, user.id);
    let ft = Fossil::count(&*conn);
    let rt = Recipe::count(&*conn);
    let at = Art::count(&*conn);
    let context = IndexContext { user: user, fossils: f, recipes: r, arts: a,
                                 ftot: ft, rtot: rt, atot: at };
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
    let id: i32 = alias.id.parse().unwrap();
    set_user_alias(&*conn, id, &alias.alias);
    Redirect::to("/")
}

// Item routes
// /edit/<item>
#[get("/<item>")]
fn edit(item: String, conn: Conn, cookies: Cookies) -> Template {
    match item.as_str() {
        "fossil" =>  {
            let (x, y, z) = get_fossil_data(&conn, &cookies, false);
            let c = SelfContext{ user: x, items: y, owned: z, itype: item };
            Template::render("edit", &c)
        },
        "recipe" =>  { 
            let (x, y, z) = get_categorized_recipe_data(&conn, &cookies);
            let c = RecipeContext{ user: x, recipes: y, owned: z };
            Template::render("editrecipes", &c)
        },
        "art" | _ => { 
            let (x, y, z) = get_art_data(&conn, &cookies, false);
            let c = SelfContext{ user: x, items: y, owned: z, itype: item };
            Template::render("edit", &c)
        }
    }
}

// /report/<item>
#[get("/<item>")]
fn report(item: String, conn: Conn, cookies: Cookies) -> Template {
    match item.as_str() {
        "fossil" =>  {
            let (x, y, z) = get_fossil_data(&conn, &cookies, false);
            let c = SelfContext{ user: x, items: y, owned: z, itype: item };
            Template::render("report", &c)
        },
        "recipe" =>  {
            let (x, y, z) = get_recipe_data(&conn, &cookies, false);
            let c = SelfContext{ user: x, items: y, owned: z, itype: item };
            Template::render("report", &c)
        },
        "art" | _ => {
            let (x, y, z) = get_art_data(&conn, &cookies, false);
            let c = SelfContext{ user: x, items: y, owned: z, itype: item };
            Template::render("report", &c)
        }
    }
}

// /all/<item>
#[get("/<item>")]
fn all(item: String, conn: Conn, cookies: Cookies) -> Template {
    let users = load_users(&*conn);
    match item.as_str() {
        "fossil" =>  {
            let (_, y, z) = get_fossil_data(&conn, &cookies, true);
            let c = AllContext{ users: users, items: y, owned: z, itype: item };
            Template::render("allreport", &c)
        },
        "recipe" =>  {
            let (_, y, z) = get_recipe_data(&conn, &cookies, true);
            let c = AllContext{ users: users, items: y, owned: z, itype: item };
            Template::render("allreport", &c)
        },
        "art" | _ => {
            let (_, y, z) = get_art_data(&conn, &cookies, true);
            let c = AllContext{ users: users, items: y, owned: z, itype: item };
            Template::render("allreport", &c)
        }
    }
}

// /whogot/<item>
#[get("/<item>")]
fn whogot(item: String, conn: Conn, cookies: Cookies) -> Template {
    let users = load_users(&*conn);
    match item.as_str() {
        "fossil" =>  {
            let (x, y, z) = get_fossil_data(&conn, &cookies, true);
            let c = got_f(x, users, y, z);
            let context = MapContext { map: c, itype: item };
            Template::render("gotreport", &context)
        },
        "recipe" =>  {
            let (x, y, z) = get_recipe_data(&conn, &cookies, true);
            let c = got_r(x, users, y, z);
            let context = MapContext { map: c, itype: item };
            Template::render("gotreport", &context)
        },
        "art" | _ => {
            let (x, y, z) = get_art_data(&conn, &cookies, true);
            let c = got_a(x, users, y, z);
            let context = MapContext { map: c, itype: item };
            Template::render("gotreport", &context)
        }
    }
}

// /whoneed/<item>
#[get("/<item>")]
fn whoneed(item: String, conn: Conn, cookies: Cookies) -> Template {
    let users = load_users(&*conn);
    match item.as_str() {
        "fossil" =>  {
            let (x, y, z) = get_fossil_data(&conn, &cookies, true);
            let c = need_f(x, users, y, z);
            let context = MapContext { map: c, itype: item };
            Template::render("needreport", &context)
        },
        "recipe" =>  {
            let (x, y, z) = get_recipe_data(&conn, &cookies, true);
            let c = need_r(x, users, y, z);
            let context = MapContext { map: c, itype: item };
            Template::render("needreport", &context)
        },
        "art" | _ => {
            let (x, y, z) = get_art_data(&conn, &cookies, true);
            let c =need_a(x, users, y, z);
            let context = MapContext { map: c, itype: item };
            Template::render("needreport", &context)
        }
    }
}

// Routes for saving form data
#[post("/fossil", data = "<data>")]
fn fsave(conn: Conn, cookies: Cookies, data: Form<EditForm>) -> Redirect {
    // Parse the jsonified owned/extra strings
    let o: BTreeMap<i32, bool> = serde_json::from_str(&data.oj).unwrap();
    let mut e: BTreeMap<i32, i32> = serde_json::from_str(&data.xj).unwrap();

    // Delete from 'e' wherever 'o' has false
    for (idx, stat) in &o {
        if *stat == true { continue; }
        let _ = e.remove(idx);
    }
    // Retrieve the user record and owned fossil records
    let user = jamie_please(&conn, &cookies);
    let owned = Ownedfossil::load(&*conn, user.id);
    let mut newfossils: Vec<NewOwnedfossil> = Vec::new();
    let mut updates: Vec<(i32, i32)> = Vec::new();

    for (id, cnt) in e {
        if let Some(existing) = owned.iter().find(|e| e.item_id == id) {
            // Check for updates, push into updates vec if needed
            if existing.extra == cnt { continue; }
            else {
                updates.push((existing.id, cnt));
            }
        } else {
            // Generate NewOwnedFossil and push into vec
            let nof = NewOwnedfossil {
                user_id: user.id,
                item_id: id,
                extra: cnt };
            newfossils.push(nof);
        }
    }

    // Send to database
    NewOwnedfossil::batch_create(&*conn, newfossils);
    Ownedfossil::update(&*conn, updates);

    Redirect::to("/edit/fossil")
}

#[post("/recipe")]
fn rsave() {}

#[post("/art")]
fn asave() {}

// LAUNCH DAT THING
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, dologin, login, newuser, alias, setalias, uuidlogin, logout])
        .mount("/", StaticFiles::from("public"))
        .mount("/edit", routes![edit])
        .mount("/report", routes![report])
        .mount("/all", routes![all])
        .mount("/whogot", routes![whogot])
        .mount("/whoneed", routes![whoneed])
        .mount("/save", routes![fsave, rsave, asave])
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
        User { id: -1, username: "-1".to_string(), alias: "-1".to_string() }
    }
}

// Since we can't use traits as return values here, just make an instance of
// 'get item data' for each Item type.
fn get_fossil_data(conn: &Conn, cookies: &Cookies, all: bool)
        -> (User, Vec<Fossil>, Vec<Ownedfossil>) {
   let user = jamie_please(&conn, &cookies);
   let items = Fossil::load(&*conn);
   let owned = match all {
       true =>  { Ownedfossil::load_all(&*conn) },
       false => { Ownedfossil::load(&*conn, user.id) }
   };
   (user, items, owned)
}

// This is more special now since we're using tabs to separate categories
fn get_categorized_recipe_data(conn: &Conn, cookies: &Cookies)
        -> (User, BTreeMap<String, Vec<Recipe>>, Vec<Ownedrecipe>) {
   let user = jamie_please(&conn, &cookies);
   let items = recipes_by_category(&*conn);
   let owned = Ownedrecipe::load(&*conn, user.id);
   (user, items, owned)
}

fn get_recipe_data(conn: &Conn, cookies: &Cookies, all: bool)
        -> (User, Vec<Recipe>, Vec<Ownedrecipe>) {
   let user = jamie_please(&conn, &cookies);
   let items = Recipe::load(&*conn);
   let owned = match all {
       true =>  { Ownedrecipe::load_all(&*conn) },
       false => { Ownedrecipe::load(&*conn, user.id) }
   };
   (user, items, owned)
}

fn get_art_data(conn: &Conn, cookies: &Cookies, all: bool)
        -> (User, Vec<Art>, Vec<Ownedart>) {
   let user = jamie_please(&conn, &cookies);
   let items = Art::load(&*conn);
   let owned = match all {
       true =>  { Ownedart::load_all(&*conn) },
       false => { Ownedart::load(&*conn, user.id) }
   };
   (user, items, owned)
}
