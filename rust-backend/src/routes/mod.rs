use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AiTool {
    name: String,
    company: String,
    description: String,
    image: Option<String>,
    monthly_price_usd: f64,
}

fn get_ai_tools() -> Vec<AiTool> {
    vec![
        AiTool {
            name: String::from("sora"),
            company: String::from("OpenAI"),
            description: String::from("Video generation tool"),
            image: None,
            monthly_price_usd: 20.0,
        },
        AiTool {
            name: String::from("GPT_4o"),
            company: String::from("OpenAI"),
            description: String::from("Text generation LLM"),
            image: None,
            monthly_price_usd: 20.0,
        },
        AiTool {
            name: String::from("claude-3_5-sonnet"),
            company: String::from("Anthropic"),
            description: String::from("Text generation LLM"),
            image: None,
            monthly_price_usd: 20.0,
        },
        AiTool {
            name: String::from("bolt.new"),
            company: String::from("Bolt"),
            description: String::from("Full-stack code generation tool"),
            image: None,
            monthly_price_usd: 50.0,
        },
    ]
}

#[get("/ai-tools")]
async fn ai_tools() -> impl Responder {
    let tools = get_ai_tools();
    HttpResponse::Ok().json(serde_json::json!({
        "tools": tools
    }))
}

#[post("/ai-tools")]
async fn create_ai_tool(tool: web::Json<AiTool>) -> impl Responder {
    println!("Successfully created new AI tool: {}", tool.name);

    HttpResponse::Created().json(serde_json::json!({
        "tool": tool.into_inner()
    }))
}

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
