use std::error::Error;

use axum::{http::StatusCode, response::Html};
use diesel::{dsl::insert_into, prelude::*};

use crate::{models::{TokenKey, User}};

pub mod models;
pub mod schema;
pub mod secrets;
pub mod gui;

pub async fn get_html(file_path: &str) -> Result<Html<String>, StatusCode> {
    Ok(Html::from(tokio::fs::read_to_string(file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}

pub fn connect_to_db() -> Result<PgConnection, StatusCode> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let connection = PgConnection::establish(&database_url)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(connection)
}

pub fn get_all_users(connection: &mut PgConnection) -> Result<Vec<User>, StatusCode> {
    Ok(schema::users::table.load::<User>(connection).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}

pub fn get_user_by_id(user_id: i32, connection: &mut PgConnection) -> Result<User, StatusCode> {
    Ok(schema::users::table.find(user_id).first::<User>(connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}

pub fn add_user(username: &str, password: &str, connection: &mut PgConnection) -> Result<(), StatusCode> {
    let password_hash = secrets::get_password_hash(password);

    let new_user = 
        (schema::users::username.eq(username), schema::users::password_hash.eq(password_hash));

    diesel::insert_into(schema::users::table)
        .values(new_user)
        .execute(connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

pub fn get_all_tokens(connection: &mut PgConnection) -> Result<Vec<TokenKey>, StatusCode> {
    let tokens = schema::token_key::table.load::<TokenKey>(connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(tokens)
}

pub async fn authorize_token(token_key: Vec<u8>) -> Result<TokenKey, StatusCode> {
    let mut connection = connect_to_db()?;
    
    let tokens = get_all_tokens(&mut connection)?;

    for db_token in tokens {
        if db_token.expiration < chrono::Local::now().naive_local() {
            diesel::delete(schema::token_key::table.filter(schema::token_key::id.eq(db_token.id)))
                .execute(&mut connection)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else if db_token.token == token_key {
            return Ok(db_token);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}


pub async fn create_token_for_user(user: &User, connection: &mut PgConnection) -> Result<TokenKey, StatusCode> {
    let id = -1;
    let expiration = chrono::Local::now().naive_local() + chrono::Duration::days(1);
    let token = secrets::get_password_hash(&user.username).as_bytes().to_vec();

    let new_token = TokenKey { id, user_id: user.id, token, expiration };

    insert_into(schema::token_key::table).values(&new_token).execute(connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(new_token)
}