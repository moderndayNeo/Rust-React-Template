use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello world!"
    }))
}

#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Hello {}!", name)
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header();

        App::new().wrap(cors).service(hello).service(hello_name)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
