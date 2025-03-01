mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use routes::{hello, hello_name};

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
