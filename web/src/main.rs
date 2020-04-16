#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate acnhc;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_contrib::databases::rusqlite;
use acnhc::*;

// Tie a type to database
#[database("sqlite_db")]
struct MyDatabase(rusqlite::Connection);

#[derive(Serialize)]
struct Context {
    foo: i32
}

#[get("/")]
fn index(conn: MyDatabase) -> Template {
    let context = CountContext {
        furncount: 0, clothcount: 0, fishcount: 0, bugcount: 0, 
        fossilcount: 0, flowercount: 0, recipecount: 0, itemcount: 0 };

    Template::render("index", &context)
}

#[get("/furniture")]
fn furniture(conn: MyDatabase) -> Template {
    let context = Context { foo: 0 };
    Template::render("furniture", &context)
}

#[get("/clothing")]
fn clothing(conn: MyDatabase) -> Template {
    let context = Context { foo: 0 };
    Template::render("clothing", &context)
}

#[get("/wallfloors")]
fn wallfloors(conn: MyDatabase) -> Template {
    let context = Context { foo: 0 };
    Template::render("wallfloor", &context)
}

#[get("/fish")]
fn fish(conn: MyDatabase) -> Template {
    let context = Context { foo: 0 };
    Template::render("fish", &context)
}

#[get("/bugs")]
fn bugs(conn: MyDatabase) -> Template {
    let context = Context { foo: 0 };
    Template::render("bugs", &context)
}

#[get("/fossils")]
fn fossils(conn: MyDatabase) -> Template {
    let context = Context { foo: 0 };
    Template::render("fossils", &context)
}

#[get("/flowers")]
fn flowers(conn: MyDatabase) -> Template {
    let context = Context { foo: 0 };
    Template::render("flowers", &context)
}

#[get("/items")]
fn items(conn: MyDatabase) -> Template {
    let context = Context { foo: 0 };
    Template::render("items", &context)
}

#[get("/recipes")]
fn recipes(conn: MyDatabase) -> Template {
    let context = Context { foo: 0 };
    Template::render("recipes", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", StaticFiles::from("public"))
        .attach(Template::fairing())
        .attach(MyDatabase::fairing())
}

fn main() {
    rocket().launch();
}
