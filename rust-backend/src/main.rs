mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use db_utils::establish_connection;
use dotenv::dotenv;
use routes::{
    create_ai_tool, delete_ai_tool, get_ai_tool, get_ai_tools, get_ai_tools_by_params, hello,
    hello_name, update_ai_tool,
};

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
            .service(create_ai_tool)
            .service(delete_ai_tool)
            .service(get_ai_tools_by_params) // This service must go before get_ai_tool because otherwise it would be interpreted as a request for a specific tool with the ID "by_params"
            .service(get_ai_tool)
            .service(update_ai_tool)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
