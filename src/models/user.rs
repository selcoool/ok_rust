use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    sqlx::FromRow,
)]

pub struct User {

    pub id: String,

    pub name: String,

    pub email: String,
}

#[derive(Debug, Deserialize)]

pub struct CreateUser {

    pub name: String,

    pub email: String,
}