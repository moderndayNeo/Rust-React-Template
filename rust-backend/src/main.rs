mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use db_utils::establish_connection;
use dotenv::dotenv;
use routes::{get_ai_tools, hello, hello_name};

pub mod db_utils;
pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = establish_connection();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(hello)
            .service(hello_name)
            .service(get_ai_tools)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
