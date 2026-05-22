use sqlx::MySqlPool;

use uuid::Uuid;

use crate::models::brand::{
    Brand,
    CreateBrand,
};

use crate::repositories::brand_repository::{
    find_all_brands,
    insert_brand,
};

pub async fn get_brands_service(
    db: &MySqlPool,
) -> Result<Vec<Brand>, sqlx::Error> {

    find_all_brands(db).await
}

pub async fn create_brand_service(
    db: &MySqlPool,
    body: CreateBrand,
) -> Result<Brand, sqlx::Error> {

    let brand = Brand {

        id: Uuid::new_v4().to_string(),

        name: body.name,
    };

    insert_brand(db, &brand).await?;

    Ok(brand)
}