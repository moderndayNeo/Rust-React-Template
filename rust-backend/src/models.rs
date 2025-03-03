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

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::ai_tools)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AiTool {
    pub id: i32,
    pub name: String,
    pub company: String,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::ai_tools)]
pub struct NewAiTool {
    pub name: String,
    pub company: String,
    pub description: String,
    pub image_url: Option<String>,
}

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
