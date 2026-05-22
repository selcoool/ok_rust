use actix_web::web;

use crate::handlers::product_handler::{
    create_product,
    get_products,
};

pub fn product_routes(
    cfg: &mut web::ServiceConfig,
) {

    cfg.service(

        web::scope("/products")

            .route(
                "",
                web::get().to(get_products)
            )

            .route(
                "",
                web::post().to(create_product)
            )
    );
}