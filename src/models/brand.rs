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

pub struct Brand {

    pub id: String,

    pub name: String,
}

#[derive(Debug, Deserialize)]

pub struct CreateBrand {

    pub name: String,
}