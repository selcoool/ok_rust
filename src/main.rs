use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    message: String,
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: "Hello World from Rust Actix Web!".to_string(),
    })
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: "Hello from API!".to_string(),
    })
}

#[get("/api/about")]
async fn about() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "name": "Actix Web API",
        "version": "1.0.0",
        "language": "Rust",
        "edition": "2024"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;

    println!("Server running: http://127.0.0.1:{}", port);

    HttpServer::new(|| App::new().service(home).service(hello).service(about))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
