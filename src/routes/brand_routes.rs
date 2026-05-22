use actix_web::web;

use crate::handlers::brand_handler::{
    create_brand,
    get_brands,
};

pub fn brand_routes(
    cfg: &mut web::ServiceConfig,
) {

    cfg.service(

        web::scope("/brands")

            .route(
                "",
                web::get().to(get_brands)
            )

            .route(
                "",
                web::post().to(create_brand)
            )
    );
}