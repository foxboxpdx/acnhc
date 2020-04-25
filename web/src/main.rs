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
    let id: i32 = alias.id.parse().unwrap();
    set_user_alias(&*conn, id, &alias.alias);
    Redirect::to("/")
}

// Fossil routes
#[get("/edit")]
fn fedit(conn: Conn, cookies: Cookies) -> Template {
    let (user, fossils, owned) = get_fossil_data(&conn, &cookies, false);
    let context = FossilSelfContext { user: user, fossils: fossils, owned: owned };
    Template::render("fossiledit", &context)
}

#[get("/report")]
fn freport(conn: Conn, cookies: Cookies) -> Template {
    let (user, fossils, owned) = get_fossil_data(&conn, &cookies, false);
    let context = FossilSelfContext { user: user, fossils: fossils, owned: owned };
    Template::render("fossilreport", &context)
}

#[get("/allreport")]
fn fall(conn: Conn, cookies: Cookies) -> Template {
    let users = load_users(&*conn);
    let (_thisuser, fossils, owned) = get_fossil_data(&conn, &cookies, true);
    let context = FossilAllContext { users: users, fossils: fossils, owned: owned };
    Template::render("fossilallreport", &context)
}

#[get("/whogot")]
fn fwhogot(conn: Conn, cookies: Cookies) -> Template {
    let users = load_users(&*conn);
    let (thisuser, fossils, owned) = get_fossil_data(&conn, &cookies, true);
    let mut contextmap = BTreeMap::new();
    for f in fossils {
        if let Some(_) = 
                owned
                .iter()
                .find(|e| e.fossil_id == f.id && e.user_id == thisuser.id) {
            continue;
        } else {
            // filter on > 0 extras
            let known_extras = owned.iter()
                .filter(|e| e.fossil_id == f.id && e.extra > 0);
            // Get user_id, look up alias, push in Vec
            let mut aliases = Vec::new();
            for ke in known_extras {
                if let Some(u) = users
                    .iter()
                    .find(|e| e.id == ke.user_id && e.alias != "None".to_string()) {
                    aliases.push(u.alias.to_string());
                }
            }
            contextmap.insert(f.name.to_string(), aliases);
        }
    }
    Template::render("fossilgotreport", &contextmap)
}


#[get("/whoneed")]
fn fwhoneed(conn: Conn, cookies: Cookies) -> Template {
    // Foreach fossil, look in owned data for thisuser.id/fossil.id/extra > 0
    // Get userids that own it and delete them from users vec
    // Grab user aliases, push into vec, insert with fossil name into BTMap
    // Send BTMap to template renderer
    let users = load_users(&*conn);
    let (thisuser, fossils, owned) = get_fossil_data(&conn, &cookies, true);
    let mut contextmap = BTreeMap::new();
    // Before we start iterating through fossils, get a list of ALL user ids
    // and aliases, for good measure.
    let mut userids = BTreeMap::new();
    for u in users {
        if u.alias != "None".to_string() {
            userids.insert(u.id, u.alias.to_string());
        }
    }
    for f in fossils {
        if let Some(_) =
            owned.iter().find(|e| e.fossil_id == f.id && e.user_id == thisuser.id && e.extra > 0) {
                // Make a fresh copy of the userids map 
                let mut localuids = userids.clone();
                // Iterate through owned looking for this fossil_id and delete
                // any users id's from localuids
                let known_owners = owned.iter().filter(|e| e.fossil_id == f.id);
                for ko in known_owners {
                    let _ = localuids.remove(&ko.user_id);
                }
                // whoever's left in localuids needs the fossil
                // compile into a string vec and slam into contextmap
                let mut aliases = Vec::new();
                for (_u, a) in localuids {
                    aliases.push(a.to_string());
                }
                contextmap.insert(f.name.to_string(), aliases);
            }
    }
    Template::render("fossilneedreport", &contextmap)
}

// Recipe routes
#[get("/edit")]
fn redit(conn: Conn, cookies: Cookies) -> Template {
    let (user, recipes, owned) = get_recipe_data(&conn, &cookies, false);
    let context = RecipeSelfContext { user: user, recipes: recipes, owned: owned };
    Template::render("recipeedit", &context)
}

#[get("/report")]
fn rreport(conn: Conn, cookies: Cookies) -> Template {
    let (user, recipes, owned) = get_recipe_data(&conn, &cookies, false);
    let context = RecipeSelfContext { user: user, recipes: recipes, owned: owned };
    Template::render("recipereport", &context)
}

#[get("/allreport")]
fn rall(conn: Conn, cookies: Cookies) -> Template {
    let users = load_users(&*conn);
    let (thisuser, recipes, owned) = get_recipe_data(&conn, &cookies, true);
    let context = RecipeAllContext { users: users, recipes: recipes, owned: owned };
    Template::render("recipeallreport", &context)
}

#[get("/whogot")]
fn rwhogot(conn: Conn, cookies: Cookies) -> Template {
    let users = load_users(&*conn);
    let (thisuser, recipes, owned) = get_recipe_data(&conn, &cookies, true);
    let context = RecipeAllContext { users: users, recipes: recipes, owned: owned };
    Template::render("recipegotreport", &context)
}


#[get("/whoneed")]
fn rwhoneed(conn: Conn, cookies: Cookies) -> Template {
    let users = load_users(&*conn);
    let (thisuser, recipes, owned) = get_recipe_data(&conn, &cookies, true);
    let context = RecipeAllContext { users: users, recipes: recipes, owned: owned };
    Template::render("recipeneedreport", &context)
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
    let owned = load_owned_fossils(&*conn, user.id);
    let mut newfossils: Vec<NewOwnedfossil> = Vec::new();
    let mut updates: Vec<(i32, i32)> = Vec::new();

    for (fos_id, cnt) in e {
        if let Some(existing) = owned.iter().find(|e| e.fossil_id == fos_id) {
            // Check for updates, push into updates vec if needed
            if existing.extra == cnt { continue; }
            else {
                updates.push((existing.id, cnt));
            }
        } else {
            // Generate NewOwnedFossil and push into vec
            let nof = NewOwnedfossil {
                user_id: user.id,
                fossil_id: fos_id,
                extra: cnt };
            newfossils.push(nof);
        }
    }

    // Send to database
    batch_ownedfossils(&*conn, newfossils);
    update_owned(&*conn, updates);

    Redirect::to("/fossil/edit")
}

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

// Ok since this trio of statements keeps appearing lets factor it out
// the bool determines whether we get ALL owned items or just one users' items
fn get_fossil_data(conn: &Conn, cookies: &Cookies, all: bool) 
        -> (User, Vec<Fossil>, Vec<Ownedfossil>) {
    let user = jamie_please(&conn, &cookies);
    let fossils = load_fossils(&*conn);
    let mut owned = Vec::new();
    if all {
        owned = load_all_owned_fossils(&*conn);
    } else {
        owned = load_owned_fossils(&*conn, user.id);
    }
    (user, fossils, owned)
}

// Same for recipes
fn get_recipe_data(conn: &Conn, cookies: &Cookies, all: bool) 
        -> (User, Vec<Recipe>, Vec<Ownedrecipe>) {
    let user = jamie_please(&conn, &cookies);
    let recipes = load_recipes(&*conn);
    let mut owned = Vec::new();
    if all {
        owned = load_all_owned_recipes(&*conn);
    } else {
        owned = load_owned_recipes(&*conn, user.id);
    }
    (user, recipes, owned)
}
