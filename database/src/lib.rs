#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate serde_derive;
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

pub fn create_recipe<'a>(conn: &SqliteConnection, name: &'a str) -> usize {
    use schema::recipes;

    let new_recipe = NewRecipe {
        name: name
    };

    diesel::insert_into(recipes::table)
        .values(&new_recipe)
        .execute(conn)
        .expect("Error saving new recipe")
}

pub fn create_ownedfossil(conn: &SqliteConnection, u: i32, f: i32, e: i32) -> usize {
    use schema::ownedfossils;

    let new_of = NewOwnedfossil {
        user_id: u,
        fossil_id: f,
        extra: e
    };

    diesel::insert_into(ownedfossils::table)
        .values(&new_of)
        .execute(conn)
        .expect("Error saving owned fossil")
}

pub fn create_ownedrecipe(conn: &SqliteConnection, u: i32, r: i32) -> usize {
    use schema::ownedrecipes;

    let new_or = NewOwnedrecipe {
        user_id: u,
        recipe_id: r
    };

    diesel::insert_into(ownedrecipes::table)
        .values(&new_or)
        .execute(conn)
        .expect("Error saving owned recipe")
}

pub fn get_uid_from_uname(conn: &SqliteConnection, name: &str) -> i32 {
    use schema::users::dsl::*;
    let uid = users.filter(username.eq(name)).select(id).first(conn);
    match uid {
        Ok(x) => x,
        Err(_) => 0
    }
}

pub fn get_user_from_uname(conn: &SqliteConnection, name: &str) -> User {
    use schema::users::dsl::*;
    let user = users.filter(username.eq(name)).first(conn);
    match user {
        Ok(x) => x,
        Err(_) => User { id: 0, username: "0".to_string(), alias: "0".to_string() }
    }
}

pub fn count_owned_fossils(conn: &SqliteConnection, uid: i32) -> i64 {
    use schema::ownedfossils::dsl::*;
    let c = ownedfossils.filter(user_id.eq(uid)).count().get_result(conn);
    match c {
        Ok(x) => x,
        Err(_) => 0
    }
}

pub fn count_owned_recipes(conn: &SqliteConnection, uid: i32) -> i64 {
    use schema::ownedrecipes::dsl::*;
    let c = ownedrecipes.filter(user_id.eq(uid)).count().get_result(conn);
    match c {
        Ok(x) => x,
        Err(_) => 0
    }
}

pub fn set_user_alias(conn: &SqliteConnection, uid: i32, a: &str) -> bool {
    use schema::users::dsl::*;
    let res = diesel::update(users.find(uid))
                .set(alias.eq(a))
                .execute(conn);
    match res {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn load_users(conn: &SqliteConnection) -> Vec<User> {
    use schema::users::dsl::*;
    users.load::<User>(conn).expect("Error loading users")
}

pub fn load_fossils(conn: &SqliteConnection) -> Vec<Fossil> {
    use schema::fossils::dsl::*;
    fossils.load::<Fossil>(conn).expect("Error loading fossils")
}

pub fn load_owned_fossils(conn: &SqliteConnection, uid: i32) -> Vec<Ownedfossil> {
    use schema::ownedfossils::dsl::*;
    ownedfossils
        .filter(user_id.eq(uid))
        .load::<Ownedfossil>(conn)
        .expect("Error loading owned fossils")
}

pub fn load_all_owned_fossils(conn: &SqliteConnection) -> Vec<Ownedfossil> {
    use schema::ownedfossils::dsl::*;
    ownedfossils
        .load::<Ownedfossil>(conn)
        .expect("Error loading all owned fossils")
}

pub fn load_recipes(conn: &SqliteConnection) -> Vec<Recipe> {
    use schema::recipes::dsl::*;
    recipes.load::<Recipe>(conn).expect("Error loading recipes")
}

pub fn load_owned_recipes(conn: &SqliteConnection, uid: i32) -> Vec<Ownedrecipe> {
    use schema::ownedrecipes::dsl::*;
    ownedrecipes
        .filter(user_id.eq(uid))
        .load::<Ownedrecipe>(conn)
        .expect("Error loading owned recipes")
}

pub fn load_all_owned_recipes(conn: &SqliteConnection) -> Vec<Ownedrecipe> {
    use schema::ownedrecipes::dsl::*;
    ownedrecipes
        .load::<Ownedrecipe>(conn)
        .expect("Error loading all owned recipes")
}

pub fn batch_ownedfossils(conn: &SqliteConnection, records: Vec<NewOwnedfossil>) {
    use schema::ownedfossils;
    diesel::insert_into(ownedfossils::table)
        .values(&records)
        .execute(conn)
        .expect("Error saving owned fossils");
}

pub fn update_owned(conn: &SqliteConnection, records: Vec<(i32, i32)>) {
    use schema::ownedfossils::dsl::*;
    for x in records {
        diesel::update(ownedfossils.find(x.0))
                .set(extra.eq(x.1))
                .execute(conn)
                .expect("Error updating owned fossil");
    }
}

