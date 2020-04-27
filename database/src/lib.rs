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

// Define common functions for "Item" types
pub trait Item {
    fn create<'a>(conn: &SqliteConnection, name: &'a str) -> usize;
    fn load(conn: &SqliteConnection) -> Vec<Self> where Self: Sized;
    fn count(conn: &SqliteConnection) -> i64;
}

impl Item for Fossil {
    fn create<'a>(conn: &SqliteConnection, name: &'a str) -> usize {
        use schema::fossils;
        let new_fossil = NewFossil { name: name };
        diesel::insert_into(fossils::table).values(&new_fossil).execute(conn)
                .expect("error saving new fossil")
    }

    fn load(conn: &SqliteConnection) -> Vec<Fossil> {
        use schema::fossils::dsl::*;
        fossils.load::<Fossil>(conn).expect("Error loading fossils")
    }

    fn count(conn: &SqliteConnection) -> i64 {
        use schema::fossils::dsl::*;
        let c = fossils.count().get_result(conn);
        match c { Ok(x) => x, Err(_) => 0 }
    }
}

impl Item for Recipe {
    fn create<'a>(conn: &SqliteConnection, name: &'a str) -> usize {
        use schema::recipes;
        let new_r = NewRecipe { name: name };
        diesel::insert_into(recipes::table).values(&new_r).execute(conn)
                .expect("error saving new recipe")
    }

    fn load(conn: &SqliteConnection) -> Vec<Recipe> {
        use schema::recipes::dsl::*;
        recipes.load::<Recipe>(conn).expect("error loading recipes")
    }

    fn count(conn: &SqliteConnection) -> i64 {
        use schema::recipes::dsl::*;
        let c = recipes.count().get_result(conn);
        match c { Ok(x) => x, Err(_) => 0 }
    }
}

impl Item for Art {
    fn create<'a>(conn: &SqliteConnection, name: &'a str) -> usize {
        use schema::arts;
        let new_a = NewArt { name: name };
        diesel::insert_into(arts::table).values(&new_a).execute(conn)
                .expect("Error saving new arts")
    }

    fn load(conn: &SqliteConnection) -> Vec<Art> {
        use schema::arts::dsl::*;
        arts.load::<Art>(conn).expect("Error loading art")
    }

    fn count(conn: &SqliteConnection) -> i64 {
        use schema::arts::dsl::*;
        let c = arts.count().get_result(conn);
        match c { Ok(x) => x, Err(_) => 0 }
    }
}

// Define common functions for "OwnedItem" types
pub trait OwnedItem {
    fn create(conn: &SqliteConnection, u: i32, i: i32, e: i32) -> usize;
    fn load(conn: &SqliteConnection, uid: i32) -> Vec<Self> where Self: Sized;
    fn load_all(conn: &SqliteConnection) -> Vec<Self> where Self: Sized;
    fn count(conn: &SqliteConnection, uid: i32) -> i64;
    fn update(conn: &SqliteConnection, records: Vec<(i32, i32)>);
}

impl OwnedItem for Ownedfossil {
    fn create(conn: &SqliteConnection, u: i32, i: i32, e: i32) -> usize {
        use schema::ownedfossils;
        let x = NewOwnedfossil { user_id: u, item_id: i, extra: e };
        diesel::insert_into(ownedfossils::table).values(&x).execute(conn)
                .expect("Error saving owned fossil")
    }

    fn load(conn: &SqliteConnection, uid: i32) -> Vec<Ownedfossil> {
        use schema::ownedfossils::dsl::*;
        ownedfossils.filter(user_id.eq(uid)).load::<Ownedfossil>(conn)
                .expect("Error loading owned fossils")
    }

    fn load_all(conn: &SqliteConnection) -> Vec<Ownedfossil> {
        use schema::ownedfossils::dsl::*;
        ownedfossils.load::<Ownedfossil>(conn)
                .expect("Error loading all owned fossils")
    }

    fn count(conn: &SqliteConnection, uid: i32) -> i64 {
        use schema::ownedfossils::dsl::*;
        let c = ownedfossils.filter(user_id.eq(uid)).count().get_result(conn);
        match c {
            Ok(x) => x,
            Err(_) => 0
        }
    }

    fn update(conn: &SqliteConnection, records: Vec<(i32, i32)>) {
        use schema::ownedfossils::dsl::*;
        for x in records {
            diesel::update(ownedfossils.find(x.0))
                .set(extra.eq(x.1))
                .execute(conn)
                .expect("Error updating owned fossil");
        }
    }
}
impl OwnedItem for Ownedrecipe {
    fn create(conn: &SqliteConnection, u: i32, i: i32, e: i32) -> usize {
        use schema::ownedrecipes;
        let x = NewOwnedrecipe { user_id: u, item_id: i, extra: e };
        diesel::insert_into(ownedrecipes::table).values(&x).execute(conn)
                .expect("Error saving owned recipe")
    }

    fn load(conn: &SqliteConnection, uid: i32) -> Vec<Ownedrecipe> {
        use schema::ownedrecipes::dsl::*;
        ownedrecipes.filter(user_id.eq(uid)).load::<Ownedrecipe>(conn)
                .expect("Error loading owned recipes")
    }

    fn load_all(conn: &SqliteConnection) -> Vec<Ownedrecipe> {
        use schema::ownedrecipes::dsl::*;
        ownedrecipes.load::<Ownedrecipe>(conn)
                .expect("Error loading all owned recipes")
    }

    fn count(conn: &SqliteConnection, uid: i32) -> i64 {
        use schema::ownedrecipes::dsl::*;
        let c = ownedrecipes.filter(user_id.eq(uid)).count().get_result(conn);
        match c {
            Ok(x) => x,
            Err(_) => 0
        }
    }

    fn update(conn: &SqliteConnection, records: Vec<(i32, i32)>) {
        use schema::ownedrecipes::dsl::*;
        for x in records {
            diesel::update(ownedrecipes.find(x.0))
                .set(extra.eq(x.1))
                .execute(conn)
                .expect("Error updating owned recipe");
        }
    }
}
impl OwnedItem for Ownedart {
    fn create(conn: &SqliteConnection, u: i32, i: i32, e: i32) -> usize {
        use schema::ownedarts;
        let x = NewOwnedart { user_id: u, item_id: i, extra: e };
        diesel::insert_into(ownedarts::table).values(&x).execute(conn)
                .expect("Error saving owned arts")
    }

    fn load(conn: &SqliteConnection, uid: i32) -> Vec<Ownedart> {
        use schema::ownedarts::dsl::*;
        ownedarts.filter(user_id.eq(uid)).load::<Ownedart>(conn)
                .expect("Error loading owned arts")
    }

    fn load_all(conn: &SqliteConnection) -> Vec<Ownedart> {
        use schema::ownedarts::dsl::*;
        ownedarts.load::<Ownedart>(conn)
                .expect("Error loading all owned arts")
    }

    fn count(conn: &SqliteConnection, uid: i32) -> i64 {
        use schema::ownedarts::dsl::*;
        let c = ownedarts.filter(user_id.eq(uid)).count().get_result(conn);
        match c {
            Ok(x) => x,
            Err(_) => 0
        }
    }

    fn update(conn: &SqliteConnection, records: Vec<(i32, i32)>) {
        use schema::ownedarts::dsl::*;
        for x in records {
            diesel::update(ownedarts.find(x.0))
                .set(extra.eq(x.1))
                .execute(conn)
                .expect("Error updating owned arts");
        }
    }
}


// Define common functions for NewOwnedItem types
pub trait NewOwnedItem {
    fn batch_create(conn: &SqliteConnection, records: Vec<Self>) where Self: Sized;
}

impl NewOwnedItem for NewOwnedfossil {
    fn batch_create(conn: &SqliteConnection, records: Vec<NewOwnedfossil>) {
        use schema::ownedfossils;
        diesel::insert_into(ownedfossils::table).values(&records).execute(conn)
                .expect("Error saving owned fossils");
    }
}

impl NewOwnedItem for NewOwnedrecipe {
    fn batch_create(conn: &SqliteConnection, records: Vec<NewOwnedrecipe>) {
        use schema::ownedrecipes;
        diesel::insert_into(ownedrecipes::table).values(&records).execute(conn)
                .expect("Error saving owned recipes");
    }
}

impl NewOwnedItem for NewOwnedart {
    fn batch_create(conn: &SqliteConnection, records: Vec<NewOwnedart>) {
        use schema::ownedarts;
        diesel::insert_into(ownedarts::table).values(&records).execute(conn)
                .expect("Error saving owned arts");
    }
}


// Generic connection function
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


// User manipulation functions
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
