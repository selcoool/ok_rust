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

pub struct Product {

    pub id: String,

    pub name: String,

    pub price: f64,

    pub brand_id: String,
}

#[derive(Debug, Deserialize)]

pub struct CreateProduct {

    pub name: String,

    pub price: f64,

    pub brand_id: String,
}