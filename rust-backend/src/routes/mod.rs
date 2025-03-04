use crate::models::{AiTool, AiToolFilters, NewAiTool};
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
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

    let result = diesel::insert_into(ai_tools)
        .values(&new_tool)
        .execute(&mut conn);

    match result {
        Ok(_) => {
            println!("Successfully created new AI tool: {}", new_tool.name);

            HttpResponse::Created().json(serde_json::json!({
                "tool": new_tool
            }))
        }
        Err(e) => {
            println!("Error creating new AI tool: {}", e);

            // Check if the error is a unique constraint violation
            if e.to_string().contains("UNIQUE constraint failed") {
                return HttpResponse::Conflict().json(serde_json::json!({
                    "error": "Tool with that name & company already exists"
                }));
            }

            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string()
            }))
        }
    }
}

#[delete("/ai-tools/{id}")]
async fn delete_ai_tool(id_to_delete: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::ai_tools::dsl::*;
    let mut conn = pool.get().expect("Error getting connection from pool");
    let tool_id = id_to_delete.into_inner();
    // print the tool_id
    println!("Deleting tool id: {}", tool_id);

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

#[get("/ai-tools/filter")]
async fn get_ai_tools_by_params(
    query: web::Query<AiToolFilters>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::ai_tools::dsl::*;

    let mut conn = pool.get().expect("Error getting connection from pool");
    let mut query_builder = ai_tools.into_boxed();

    if let Some(company_filter) = &query.company {
        query_builder = query_builder.filter(company.eq(company_filter));
    }

    if let Some(name_filter) = &query.name {
        query_builder = query_builder.filter(name.eq(name_filter));
    }

    let tools = query_builder
        .load::<AiTool>(&mut conn)
        .expect("Error loading AI tools");

    HttpResponse::Ok().json(serde_json::json!({
        "tools": tools
    }))
}

#[get("/ai-tools/{id}")]
async fn get_ai_tool(tool_id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::ai_tools::dsl::*;
    let mut conn = pool.get().expect("Error getting connection from pool");
    let tool_id = tool_id.into_inner();
    // print the tool_id
    println!("Fetching tool with id: {}", tool_id);

    let result = ai_tools.filter(id.eq(tool_id)).first::<AiTool>(&mut conn);

    match result {
        Ok(tool) => HttpResponse::Ok().json(serde_json::json!({"tool": tool})),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({"error": e.to_string()})),
    }
}

#[put("/ai-tools/{id}")]
async fn update_ai_tool(
    tool_id: web::Path<i32>,
    pool: web::Data<DbPool>,
    updated_tool: web::Json<AiTool>,
) -> impl Responder {
    use crate::schema::ai_tools::dsl::*;
    let mut conn = pool.get().expect("Error getting connection from pool");
    let tool_id = tool_id.into_inner();
    println!("Updating tool with id: {}", tool_id);

    let result = diesel::update(ai_tools.filter(id.eq(tool_id)))
        .set(updated_tool.into_inner())
        .execute(&mut conn);

    match result {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({ "message": format!("Tool with id {} updated successfully", tool_id) }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))
        }
    }
}
