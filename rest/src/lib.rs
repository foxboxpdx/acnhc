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

    fn update<'a>(conn: &SqliteConnection, item: &Item) -> usize {
        use schema::items;
        diesel::update(items::table).set(item).execute(conn)
            .expect("Error updating item")
     }

    fn delete(conn: &SqliteConnection, did: i32) -> usize {
        use schema::items;
        use schema::items::dsl::*;
        diesel::delete(items::table.filter(id.eq(did))).execute(conn)
            .expect("Error deleting item")
     }

    // Bonus function to get a count based on pri/sub types
    fn count(conn: &SqliteConnection, pt: i32, st: i32) -> i64 {
        use schema::items::dsl::*;
        let c = items.count().get_result(conn);
        match c { Ok(x) => x, Err(_) => 0 }
    }

    // QoL extras
    fn read_by_pritype(conn: &SqliteConnection, ptype: i32) -> Vec<Item> {
        use schema::items::dsl::*;
        items
            .order(name_en.asc())
            .filter(pri_type.eq(ptype))
            .load::<Item>(conn).expect("Error loading items by pritype")
    }

    fn read_by_subtype(conn: &SqliteConnection, ptype: i32, stype: i32) -> Vec<Item> {
        use schema::items::dsl::*;
        items
            .order(name_en.asc())
            .filter(pri_type.eq(ptype))
            .filter(sub_type.eq(stype))
            .load::<Item>(conn).expect("Error loading items by subtype")
    }

}

// Define CRUD functions for OwnedItem
impl Owneditem {
    fn create(conn: &SqliteConnection, uid: i32, iid: i32, x: i32) -> usize {
        use schema::owneditems;
        let new_oi = NewOwneditem { user_id: uid, item_id: iid, extra: x };
        diesel::insert_into(owneditems::table).values(&new_oi).execute(conn)
            .expect("Error saving new owned item")
    }

    fn read(conn: &SqliteConnection, uid: i32) -> Vec<Owneditem> {
        use schema::owneditems::dsl::*;
        owneditems
            .filter(user_id.eq(uid))
            .load::<Owneditem>(conn).expect("Error loading owned items by userid")
    }

    fn update(conn: &SqliteConnection, oi: &Owneditem) -> usize {
        use schema::owneditems;
        diesel::update(owneditems::table).set(oi).execute(conn)
            .expect("Error updating owneditem")
    }

    fn delete(conn: &SqliteConnection, did: i32) -> usize {
        use schema::owneditems;
        use schema::owneditems::dsl::*;
        diesel::delete(owneditems::table.filter(id.eq(did))).execute(conn)
            .expect("Error deleting owneditem")
    }

    // Counting again
    fn count(conn: &SqliteConnection, uid: i32) -> i64 {
        use schema::owneditems;
        use schema::owneditems::dsl::*;
        let c = owneditems::table.filter(user_id.eq(uid)).count().get_result(conn);
        match c { Ok(x) => x, Err(_) => 0 }
    }

    // QoL extras
    // This is going to need some kind of cross-query that pulls pri_type and sub_type out of the
    // Items table based on the item_id in the Owneditem table
    fn count_by_type(conn: &SqliteConnection, uid: i32, pt: i32, st: i32) -> i64 {
        use schema::owneditems;
        use schema::owneditems::dsl::*;
        let c = owneditems::table.filter(user_id.eq(uid)).count().get_result(conn);
        match c { Ok(x) => x, Err(_) => 0 }
    }

}

// Define CRUD functions for User
impl User {
    fn create<'a>(conn: &SqliteConnection, uname: &'a str, pass: &'a str) -> usize {
        use schema::users;
        let new_u = NewUser { username: uname, passwd: pass };
        diesel::insert_into(users::table).values(&new_u).execute(conn)
            .expect("Error saving new user")
    }

    fn read(conn: &SqliteConnection) -> Vec<User> {
        use schema::users::dsl::*;
        users
            .order(seen.asc())
            .load::<User>(conn).expect("Error loading users")
    }

    fn update(conn: &SqliteConnection, u: &User) -> usize {
        use schema::users;
        diesel::update(users::table).set(u).execute(conn)
            .expect("Error updating user")
    }

    fn delete(conn: &SqliteConnection, did: i32) -> usize {
        use schema::users;
        use schema::users::dsl::*;
        diesel::delete(users::table.filter(id.eq(did))).execute(conn)
            .expect("Error deleting user")
    }

    // Try to load a specific user and return an error or something if not found
    fn read_by_username() {}
}

// Define CRUD functions for PrimaryType
impl Primarytype {
    fn create<'a>(conn: &SqliteConnection, tname: &'a str) -> usize {
        use schema::primarytypes;
        let new_pt = NewPrimarytype { name: tname };
        diesel::insert_into(primarytypes::table).values(&new_pt).execute(conn)
            .expect("Error saving new primary type")
    }

    fn read(conn: &SqliteConnection) -> Vec<Primarytype> {
        use schema::primarytypes::dsl::*;
        primarytypes
            .order(id.asc())
            .load::<Primarytype>(conn).expect("Error loading primarytypes")
     }

    fn update(conn: &SqliteConnection, pt: &Primarytype) -> usize {
        use schema::primarytypes;
        diesel::update(primarytypes::table).set(pt).execute(conn)
            .expect("Error updating primarytype")
    }

    fn delete(conn: &SqliteConnection, did: i32) -> usize {
        use schema::primarytypes;
        use schema::primarytypes::dsl::*;
        diesel::delete(primarytypes::table.filter(id.eq(did))).execute(conn)
            .expect("Error deleting primarytype")
    }
}

// Define CRUD functions for SubType
impl Subtype {
    fn create<'a>(conn: &SqliteConnection, tname: &'a str) -> usize {
        use schema::subtypes;
        let new_st = NewSubtype { name: tname };
        diesel::insert_into(subtypes::table).values(&new_st).execute(conn)
            .expect("Error saving new subtype")
    }

    fn read(conn: &SqliteConnection) -> Vec<Subtype> {
        use schema::subtypes::dsl::*;
        subtypes
            .order(id.asc())
            .load::<Subtype>(conn).expect("Error loading subtypes")
    }

    fn update(conn: &SqliteConnection, st: &Subtype) -> usize {
        use schema::subtypes;
        diesel::update(subtypes::table).set(st).execute(conn)
            .expect("Error updating subtype")
    }

    fn delete(conn: &SqliteConnection, did: i32) -> usize {
        use schema::subtypes;
        use schema::subtypes::dsl::*;
        diesel::delete(subtypes::table.filter(id.eq(did))).execute(conn)
            .expect("Error deleting subtype")
    }
}

// Define some bonus functions for QoL improvements
