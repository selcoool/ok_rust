use actix_web::{
    web,
    HttpResponse,
    Responder,
};

use sqlx::MySqlPool;

use crate::models::brand::CreateBrand;

use crate::services::brand_service::{
    create_brand_service,
    get_brands_service,
};

type Db = web::Data<MySqlPool>;

pub async fn get_brands(
    db: Db,
) -> impl Responder {

    match get_brands_service(
        db.get_ref(),
    )
    .await
    {

        Ok(brands) => {
            HttpResponse::Ok().json(brands)
        }

        Err(err) => {

            HttpResponse::InternalServerError()
                .body(err.to_string())
        }
    }
}

pub async fn create_brand(
    db: Db,
    body: web::Json<CreateBrand>,
) -> impl Responder {

    match create_brand_service(
        db.get_ref(),
        body.into_inner(),
    )
    .await
    {

        Ok(brand) => {
            HttpResponse::Created().json(brand)
        }

        Err(err) => {

            HttpResponse::InternalServerError()
                .body(err.to_string())
        }
    }
}