use axum::{
    Json, Router, http::{HeaderMap, StatusCode}, response::{Html, IntoResponse}, routing::{get, post},
};
use home_menu::{connect_to_db, get_html, gui, models::User, secrets::{self, get_password_hash, user_from_auth}};

#[tokio::main]
async fn main() {
    let ui = gui::WindowManager::new();

    // let connection = home_menu::connect_to_db().expect("Failed to connect to the database");
    let app = Router::new()
        .route("/", get(main_page))
        .route("/wake", post(wake_gui))
        .route("/add_user", post(add_user))
        .route("/get_users", get(users_page))
        .route("/login", get(login_page))
    ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn main_page() -> Result<Html<String>, StatusCode> {
    let html = get_html("static/test.html").await?;
    Ok(html)
}

async fn users_page(head: HeaderMap) -> Result<Json<Vec<String>>, StatusCode> {
    let mut connection = connect_to_db()?;
    let _ = user_from_auth(&head, &mut connection)?;

    let users = home_menu::get_all_users(&mut connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users.into_iter().map(|user| user_to_json_string(&user)).collect()))
}

async fn login_page() -> Result<Html<String>, StatusCode> {
    let html = get_html("static/401.html").await?;
    Ok(html)
}

async fn wake_gui(head: HeaderMap, body: Json<User>) -> Result<&'static str, StatusCode> {
    let mut connection = home_menu::connect_to_db()?;

    let login_user = user_from_auth(&head, &mut connection)?;

    // let users = home_menu::get_all_users(&mut connection.unwrap())
        // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok("Hello World")
}

#[derive(serde::Deserialize)]
struct AddUserRequest {
    username: String,
    password: String,
}

pub fn user_to_json_string(user: &home_menu::models::User) -> String {
    format!(
        "{{\"id\": {}, \"username\": \"{}\", \"password_hash\": \"{}\"}}",
        user.id, user.username, user.password_hash
    )
}


async fn add_user(head: HeaderMap, json: Json<AddUserRequest>) -> Result<String, StatusCode> {
    if json.username.is_empty() || json.password.is_empty() {
        println!("Invalid request: missing username, password_hash, or token.");
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut connection = connect_to_db()?;
    let _ = user_from_auth(&head, &mut connection)?;

    let _ = home_menu::add_user(
        json.username.as_str(), 
        secrets::get_password_hash(json.password.as_str()).as_str(),
        &mut connection
    ).map_err(|_| return StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(format!("User {} added successfully.", json.username))
}