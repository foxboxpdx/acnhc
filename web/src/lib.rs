// Define helper functions and structs for ACNHC
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate acnhc_db;

use acnhc_db::models::*;
use acnhc_db::*;
use std::collections::BTreeMap;

// Form structs //

// Login struct
#[derive(FromForm)]
pub struct Userid {
    pub id: String,
}

// Alias setting struct
#[derive(FromForm)]
pub struct Alias {
    pub id: String,
    pub alias: String,
}

// Edit form struct
// XJ and OJ are json-encoded hashmaps
#[derive(FromForm, Debug)]
pub struct EditForm {
    pub owned: String,
    pub extra: String,
    pub oj: String,
    pub xj: String
}

// Context structs //

// Empty context for testing
#[derive(Serialize)]
pub struct EmptyContext {}

// Basic struct just containing a User
#[derive(Serialize)]
pub struct UserContext {
  pub user: User
}

// Index/main context
#[derive(Serialize)]
pub struct IndexContext {
    pub user:    String,
    pub fossils: i64,
    pub recipes: i64,
    pub arts:    i64
}

// btreemap context
#[derive(Serialize)]
pub struct MapContext {
    pub map: BTreeMap<String, Vec<String>>
}

// Single user data
#[derive(Serialize)]
pub struct SelfContext<T, U> where T: Item, U: OwnedItem {
    pub user:  User,
    pub items: Vec<T>,
    pub owned: Vec<U>,
}

// All user data
#[derive(Serialize)]
pub struct AllContext<T, U> where T: Item, U: OwnedItem {
    pub users: Vec<User>,
    pub items: Vec<T>,
    pub owned: Vec<U>,
}

// Helper functions
pub fn got_f(thisuser: User, users: Vec<User>, items: Vec<Fossil>, owned: Vec<Ownedfossil>) -> BTreeMap<String, Vec<String>> {
    let mut contextmap = BTreeMap::new();
    for f in items {
        if let Some(_) =
                owned
                .iter()
                .find(|e| e.item_id == f.id && e.user_id == thisuser.id) {
            continue;
        } else {
            // filter on > 0 extras
            let known_extras = owned.iter()
                .filter(|e| e.item_id == f.id && e.extra > 0);
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
    contextmap
}

pub fn got_r(thisuser: User, users: Vec<User>, items: Vec<Recipe>, owned: Vec<Ownedrecipe>) -> BTreeMap<String, Vec<String>> {
    let mut contextmap = BTreeMap::new();
    for f in items {
        if let Some(_) =
                owned
                .iter()
                .find(|e| e.item_id == f.id && e.user_id == thisuser.id) {
            continue;
        } else {
            // filter on > 0 extras
            let known_extras = owned.iter()
                .filter(|e| e.item_id == f.id && e.extra > 0);
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
    contextmap
}

pub fn got_a(thisuser: User, users: Vec<User>, items: Vec<Art>, owned: Vec<Ownedart>) -> BTreeMap<String, Vec<String>> {
    let mut contextmap = BTreeMap::new();
    for f in items {
        if let Some(_) =
                owned
                .iter()
                .find(|e| e.item_id == f.id && e.user_id == thisuser.id) {
            continue;
        } else {
            // filter on > 0 extras
            let known_extras = owned.iter()
                .filter(|e| e.item_id == f.id && e.extra > 0);
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
    contextmap
}

pub fn need_f(thisuser: User, users: Vec<User>, items: Vec<Fossil>, owned: Vec<Ownedfossil>) -> BTreeMap<String, Vec<String>> {
    let mut contextmap = BTreeMap::new();
    let mut userids = BTreeMap::new();
    for u in users {
        if u.alias != "None".to_string() {
            userids.insert(u.id, u.alias.to_string());
        }
    }
    for f in items {
        if let Some(_) =
            owned.iter().find(|e| e.item_id == f.id && e.user_id == thisuser.id && e.extra > 0) {
                // Make a fresh copy of the userids map
                let mut localuids = userids.clone();
                // Iterate through owned looking for this fossil_id and delete
                // any users id's from localuids
                let known_owners = owned.iter().filter(|e| e.item_id == f.id);
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
    contextmap
}

pub fn need_r(thisuser: User, users: Vec<User>, items: Vec<Recipe>, owned: Vec<Ownedrecipe>) -> BTreeMap<String, Vec<String>> {
    let mut contextmap = BTreeMap::new();
    let mut userids = BTreeMap::new();
    for u in users {
        if u.alias != "None".to_string() {
            userids.insert(u.id, u.alias.to_string());
        }
    }
    for f in items {
        if let Some(_) =
            owned.iter().find(|e| e.item_id == f.id && e.user_id == thisuser.id && e.extra > 0) {
                // Make a fresh copy of the userids map
                let mut localuids = userids.clone();
                // Iterate through owned looking for this fossil_id and delete
                // any users id's from localuids
                let known_owners = owned.iter().filter(|e| e.item_id == f.id);
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
    contextmap
}

pub fn need_a(thisuser: User, users: Vec<User>, items: Vec<Art>, owned: Vec<Ownedart>) -> BTreeMap<String, Vec<String>> {
    let mut contextmap = BTreeMap::new();
    let mut userids = BTreeMap::new();
    for u in users {
        if u.alias != "None".to_string() {
            userids.insert(u.id, u.alias.to_string());
        }
    }
    for f in items {
        if let Some(_) =
            owned.iter().find(|e| e.item_id == f.id && e.user_id == thisuser.id && e.extra > 0) {
                // Make a fresh copy of the userids map
                let mut localuids = userids.clone();
                // Iterate through owned looking for this fossil_id and delete
                // any users id's from localuids
                let known_owners = owned.iter().filter(|e| e.item_id == f.id);
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
    contextmap
}
