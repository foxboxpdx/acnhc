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

// Generic connection function
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Define CRUD functions for Item
impl Item {
    fn create<'a>(conn: &SqliteConnection, en: &'a str, jp: &'a str, pt: i32, st: i32) -> usize {
        use schema::items;
        let new_item = NewItem { name_en: en, name_jp: jp, pri_type: pt, sub_type: st};
        diesel::insert_into(items::table).values(&new_item).execute(conn)
            .expect("error saving new item")
    }

    fn read(conn: &SqliteConnection) -> Vec<Item> {
        use schema::items::dsl::*;
        items
            .order(name_en.asc())
            .load::<Item>(conn).expect("Error loading items")
    }

    fn update() { }

    fn delete() { }

    // Bonus function to get a count based on pri/sub types
    fn count(conn: &SqliteConnection, pt: i32, st: i32) -> i64 {
        use schema::items::dsl::*;
        let c = items.count().get_result(conn);
        match c { Ok(x) => x, Err(_) => 0 }
    }

    // QoL extras
    fn read_by_pritype() {}

    fn read_by_subtype() {}
    
}

// Define CRUD functions for OwnedItem
impl OwnedItem {
    fn create() {}

    fn read() {}

    fn update() {}

    fn delete() {}

    // Counting again
    fn count() {}

    // QoL extras
    fn count_by_type() {}

}

// Define CRUD functions for User
impl User {
    fn create() {}

    fn read() {}

    fn update() {}

    fn delete() {}
}

// Define CRUD functions for PrimaryType
impl PrimaryType {
    fn create() {}

    fn read() { }

    fn update() {}

    fn delete() {}
}

// Define CRUD functions for SubType
impl SubType {
    fn create() {}

    fn read() {}

    fn update() {}

    fn delete() {}
}

// Define some bonus functions for QoL improvements
