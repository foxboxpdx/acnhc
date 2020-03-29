#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate acnhc;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_contrib::databases::rusqlite;
use acnhc::*;

// Tie a type to database
#[database("my_db")]
struct MyDatabase(rusqlite::Connection);

#[get("/")]
fn index(conn: MyDatabase) -> Template {
    let context = CountContext {
        furncount: 0, furnvarcount: 0, clothcount: 0, clothvarcount: 0,
        fishcount: 0, bugcount: 0, fossilcount: 0, flowercount: 0,
        recipecount: 0, itemcount: 0 };

    Template::render("index", &context)
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
