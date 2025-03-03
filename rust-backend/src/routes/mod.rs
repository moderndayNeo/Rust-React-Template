use crate::models::{AiTool, NewAiTool};
use actix_web::{HttpResponse, Responder, delete, get, post, web};
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

#[post("/ai-tools")]
async fn create_ai_tool(tool: web::Json<NewAiTool>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::ai_tools::dsl::*;

    let new_tool = tool.into_inner();

    let mut conn = pool.get().expect("Error getting connection from pool");

    diesel::insert_into(ai_tools)
        .values(&new_tool)
        .execute(&mut conn)
        .expect("Error inserting new AI tool");

    println!("Successfully created new AI tool: {}", new_tool.name);

    HttpResponse::Created().json(serde_json::json!({
        "tool": new_tool
    }))
}

#[delete("/ai-tools/{id}")]
async fn delete_ai_tool(id_to_delete: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::ai_tools::dsl::*;
    let mut conn = pool.get().expect("Error getting connection from pool");
    let tool_id = id_to_delete.into_inner();
    // print the tool_id
    println!("Tool id: {}", tool_id);

    let result = diesel::delete(ai_tools.filter(id.eq(tool_id))).execute(&mut conn);

    match result {
        Ok(num) => {
            if num > 0 {
                HttpResponse::Ok().json(serde_json::json!({"status": "success"}))
            } else {
                HttpResponse::NotFound()
                    .json(serde_json::json!({"error": "Tool not found with id"}))
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

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
