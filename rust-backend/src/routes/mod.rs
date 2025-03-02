use actix_web::{HttpResponse, Responder, get, web};
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
