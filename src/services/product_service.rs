use sqlx::MySqlPool;

use uuid::Uuid;

use crate::models::product::{
    CreateProduct,
    Product,
};

use crate::repositories::product_repository::{
    find_all_products,
    insert_product,
};

pub async fn get_products_service(
    db: &MySqlPool,
) -> Result<Vec<Product>, sqlx::Error> {

    find_all_products(db).await
}

pub async fn create_product_service(
    db: &MySqlPool,
    body: CreateProduct,
) -> Result<Product, sqlx::Error> {

    let product = Product {

        id: Uuid::new_v4().to_string(),

        name: body.name,

        price: body.price,

        brand_id: body.brand_id,
    };

    insert_product(db, &product).await?;

    Ok(product)
}