use actix_web::{
    web,
    HttpResponse,
    Responder,
};

use sqlx::MySqlPool;

use crate::models::product::CreateProduct;

use crate::services::product_service::{
    create_product_service,
    get_products_service,
};

type Db = web::Data<MySqlPool>;

pub async fn get_products(
    db: Db,
) -> impl Responder {

    match get_products_service(
        db.get_ref(),
    )
    .await
    {

        Ok(products) => {
            HttpResponse::Ok().json(products)
        }

        Err(err) => {

            HttpResponse::InternalServerError()
                .body(err.to_string())
        }
    }
}

pub async fn create_product(
    db: Db,
    body: web::Json<CreateProduct>,
) -> impl Responder {

    match create_product_service(
        db.get_ref(),
        body.into_inner(),
    )
    .await
    {

        Ok(product) => {
            HttpResponse::Created().json(product)
        }

        Err(err) => {

            HttpResponse::InternalServerError()
                .body(err.to_string())
        }
    }
}