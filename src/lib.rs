use axum::{http::StatusCode, response::Html};
use diesel::prelude::*;

pub mod models;
pub mod schema;

pub async fn get_html(file_path: &str) -> Result<Html<String>, StatusCode> {
    Ok(Html::from(tokio::fs::read_to_string(file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

fn connect_to_db() -> Result<PgConnection, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&database_url)?)
}