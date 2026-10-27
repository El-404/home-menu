use std::error::Error;

use axum::{http::{HeaderMap, StatusCode}, response::Html};
use diesel::prelude::*;

use crate::models::User;

pub mod models;
pub mod schema;
pub mod secrets;
pub mod gui;

pub async fn get_html(file_path: &str) -> Result<Html<String>, StatusCode> {
    Ok(Html::from(tokio::fs::read_to_string(file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

pub fn connect_to_db() -> Result<PgConnection, Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&database_url)?)
}

pub fn connect_with_token(token: &HeaderMap) -> Result<PgConnection, Box<dyn Error>> {
    if !secrets::is_valid_token(token) {
        return Err("Invalid token".into());
    }
    connect_to_db()
}

pub fn get_all_users(connection: &mut PgConnection) -> Result<Vec<User>, Box<dyn Error>> {
    let users = 
    schema::users::table.load::<User>(connection)?;

    Ok(users)
}

pub fn get_user_by_id(user_id: i32, connection: &mut PgConnection) -> Result<User, Box<dyn Error>> {
    let user = schema::users::table.find(user_id).first::<User>(connection)?;

    Ok(user)
}

pub fn add_user(username: &str, password: &str, connection: &mut PgConnection) -> Result<(), Box<dyn Error>> {
    let password_hash = secrets::get_password_hash(password);

    let new_user = 
        (schema::users::username.eq(username), schema::users::password_hash.eq(password_hash));

    diesel::insert_into(schema::users::table)
        .values(new_user)
        .execute(connection)?;

    Ok(())
}