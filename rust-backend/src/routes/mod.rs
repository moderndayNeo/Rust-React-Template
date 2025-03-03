use crate::models::AiTool;
use actix_web::{HttpResponse, Responder, get, web};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

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

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/ai-tools")]
async fn get_ai_tools(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::ai_tools::dsl::*;

    let mut conn = pool.get().expect("Error getting connection from pool");

    let tools = ai_tools
        .load::<AiTool>(&mut conn)
        .expect("Error loading AI tools");

    HttpResponse::Ok().json(serde_json::json!({
        "tools": tools
    }))
}

// #[get("/houses")]
// async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
//     let houses = web::block(move || {
//         let mut conn = pool.get()?; // <------------
//         find_all(&mut conn)         // <------------
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     Ok(HttpResponse::Ok().json(houses))
// }

// fn find_all(conn: &mut PgConnection) -> Result<Vec<House>, DbError> {
//                 // ^^^ <------------
//     use crate::schemas::common::houses::houses::dsl::*;

//     let items = houses.load::<House>(conn)?; // <------------
//     Ok(items)
// }

// #[post("/ai-tools")]
// async fn create_ai_tool(
//     tool: web::Json<AiTool>,
//     pool: web::Data<SqliteConnection>,
// ) -> impl Responder {
//     use crate::schema::ai_tools;

//     let new_tool = tool.into_inner();

//     diesel::insert_into(ai_tools::table)
//         .values(&new_tool)
//         .execute(&**pool)
//         .expect("Error inserting new AI tool");

//     println!("Successfully created new AI tool: {}", new_tool.name);

//     HttpResponse::Created().json(serde_json::json!({
//         "tool": new_tool
//     }))
// }

// Each endpoint needs to be registered using .service()
// The macro attributes (#[get], #[post], etc.) define the HTTP method and path
// Use web::Json<T> for handling JSON payloads
// Use web::Path<T> for path parameters

// #[get("/ai-tools/{id}")]
// async fn get_ai_tool(id: web::Path<String>) -> impl Responder {
//     // Implement get single tool logic
//     HttpResponse::Ok().json(serde_json::json!({ "message": "Get single tool" }))
// }

// #[put("/ai-tools/{id}")]
// async fn update_ai_tool(id: web::Path<String>, tool: web::Json<AiTool>) -> impl Responder {
//     // Implement update logic
//     HttpResponse::Ok().json(serde_json::json!({ "message": "Update tool" }))
// }

// #[delete("/ai-tools/{id}")]
// async fn delete_ai_tool(id: web::Path<String>) -> impl Responder {
//     // Implement delete logic
//     HttpResponse::Ok().json(serde_json::json!({ "message": "Delete tool" }))
// }

// #[post("/ai-tools")]
// async fn create_ai_tool(
//     tool: web::Json<AiTool>,
//     db: web::Data<SqliteConnection>,
// ) -> Result<impl Responder, Error> {
//     // Store in database
//     let stored_tool = db.insert_ai_tool(&tool).await?;

//     println!("Successfully created new AI tool: {}", stored_tool.name);

//     Ok(HttpResponse::Created().json(serde_json::json!({
//         "tool": stored_tool
//     })))
// }
