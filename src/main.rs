mod db;
mod models;
mod repositories;
mod services;
mod handlers;
mod routes;

use actix_cors::Cors;

use actix_web::{
    web,
    App,
    HttpServer,
};

use db::connection::connect_db;

use routes::{
    brand_routes::brand_routes,
    product_routes::product_routes,
    user_routes::user_routes,
};

#[actix_web::main]

async fn main() -> std::io::Result<()> {

    let pool = connect_db().await;

    println!("SERVER RUNNING");

    HttpServer::new(move || {

        App::new()

            .wrap(Cors::permissive())

            .app_data(
                web::Data::new(pool.clone())
            )

            .configure(user_routes)

            .configure(product_routes)

            .configure(brand_routes)
    })

    .bind(("127.0.0.1", 8080))?

    .run()

    .await
}