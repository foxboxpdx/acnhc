// Helper functions and such for the web side of things
use crate::models::*;
use crate::*;

// Define structs for http input
// Incoming data for creating or authenticating a user
#[derive(Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub passwd: String
}

// Incoming data requesting a specific set of items/owneditems
#[derive(Deserialize)]
pub struct DataRequest {
    pub userid: i32,
    pub primary: String,
    pub subtype: String
}

// Incoming data for updating the database
// Userid must be set.  If ownedid is unset, we need to create a new entry
// in the owneditems table.  
#[derive(Deserialize)]
pub struct UpdateRequest {
    pub userid: i32,
    pub itemid: i32,
    pub ownedid: i32,
    pub extras: i32
}

// Define structs for http output
// Outgoing user authentication confirmation
#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    pub userid: i32,
    pub token: String
}

// Outgoing data containing items/owneditems
#[derive(Serialize)]
pub struct DataResponse {
    pub items: Vec<Item>,
    pub owned: Vec<Owneditem>
}

// Outgoing update confirmation data
#[derive(Serialize)]
pub struct UpdateResponse {
    pub ownedid: i32,
    pub extras: i32
}
