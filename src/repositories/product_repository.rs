use sqlx::MySqlPool;

use crate::models::product::Product;

pub async fn find_all_products(
    db: &MySqlPool,
) -> Result<Vec<Product>, sqlx::Error> {

    sqlx::query_as::<_, Product>(
        "SELECT * FROM products"
    )
    .fetch_all(db)
    .await
}

pub async fn insert_product(
    db: &MySqlPool,
    product: &Product,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        "INSERT INTO products(
            id,
            name,
            price,
            brand_id
         )
         VALUES(?,?,?,?)"
    )
    .bind(&product.id)
    .bind(&product.name)
    .bind(product.price)
    .bind(&product.brand_id)
    .execute(db)
    .await?;

    Ok(())
}