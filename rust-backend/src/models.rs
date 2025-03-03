use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::ai_tools)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AiTool {
    pub id: Option<i32>,
    pub name: String,
    pub company: String,
    pub description: String,
    pub image_url: Option<String>,
}

// fn get_ai_tools() -> Vec<AiTool> {
//     vec![
//         AiTool {
//             name: String::from("sora"),
//             company: String::from("OpenAI"),
//             description: String::from("Video generation tool"),
//             image: None,
//             monthly_price_usd: 20.0,
//         },
//         AiTool {
//             name: String::from("GPT_4o"),
//             company: String::from("OpenAI"),
//             description: String::from("Text generation LLM"),
//             image: None,
//             monthly_price_usd: 20.0,
//         },
//         AiTool {
//             name: String::from("claude-3_5-sonnet"),
//             company: String::from("Anthropic"),
//             description: String::from("Text generation LLM"),
//             image: None,
//             monthly_price_usd: 20.0,
//         },
//         AiTool {
//             name: String::from("bolt.new"),
//             company: String::from("Bolt"),
//             description: String::from("Full-stack code generation tool"),
//             image: None,
//             monthly_price_usd: 50.0,
//         },
//     ]
// }


#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::martial_arts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MartialArt {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image_url: Option<String>,
    pub country_of_origin: String,
    pub monthly_price_usd: f32,
}
