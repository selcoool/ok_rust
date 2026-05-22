use sqlx::MySqlPool;

use crate::models::brand::Brand;

pub async fn find_all_brands(
    db: &MySqlPool,
) -> Result<Vec<Brand>, sqlx::Error> {

    sqlx::query_as::<_, Brand>(
        "SELECT * FROM brands"
    )
    .fetch_all(db)
    .await
}

pub async fn insert_brand(
    db: &MySqlPool,
    brand: &Brand,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        "INSERT INTO brands(id,name)
         VALUES(?,?)"
    )
    .bind(&brand.id)
    .bind(&brand.name)
    .execute(db)
    .await?;

    Ok(())
}