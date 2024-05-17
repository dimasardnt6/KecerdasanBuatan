use serde::{Serialize,Deserialize};

#[derive(Serialize,Default)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

// A struct for query parameters
#[derive(Deserialize)]
pub struct Page {
    pub number: u32,
}


// A struct for the JSON body
#[derive(Deserialize)]
pub struct Item {
    pub title: String,
}

#[derive(Serialize)]
pub struct Userx {
    pub id: u64,
    pub name: String,
}