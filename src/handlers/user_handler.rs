use actix_web::{
    web,
    HttpResponse,
    Responder,
};

use sqlx::MySqlPool;

use crate::models::user::CreateUser;

use crate::services::user_service::{
    create_user_service,
    get_users_service,
};

type Db = web::Data<MySqlPool>;

pub async fn get_users(
    db: Db,
) -> impl Responder {

    match get_users_service(
        db.get_ref(),
    )
    .await
    {

        Ok(users) => {
            HttpResponse::Ok().json(users)
        }

        Err(err) => {

            HttpResponse::InternalServerError()
                .body(err.to_string())
        }
    }
}

pub async fn create_user(
    db: Db,
    body: web::Json<CreateUser>,
) -> impl Responder {

    match create_user_service(
        db.get_ref(),
        body.into_inner(),
    )
    .await
    {

        Ok(user) => {
            HttpResponse::Created().json(user)
        }

        Err(err) => {

            HttpResponse::InternalServerError()
                .body(err.to_string())
        }
    }
}