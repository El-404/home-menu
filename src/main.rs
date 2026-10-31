use axum::{
    Json, Router, http::{HeaderMap, StatusCode, }, response::{Html, IntoResponse, Redirect, Response}, routing::{get, post},
};
use axum_extra::{TypedHeader, extract::{CookieJar, cookie::Cookie}, headers::{Authorization, authorization::Bearer}};
use home_menu::{connect_to_db, get_html, gui, secrets::get_password_hash};

#[tokio::main]
async fn main() {
    // let mut connection = connect_to_db().expect("Failed to connect to the database");
    // add_user("Ethan", "1234567890", &mut connection);
    let future_internet = tokio::spawn(start_page());
    let _ui = gui::start_gui();
    future_internet.await.unwrap();
}

async fn start_page() {
    let app = Router::new()
        .route("/", get(main_page))
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

    let users = home_menu::get_all_users(&mut connection)?;

    Ok(Json(users.into_iter().map(|user| user.to_json_string()).collect()))
}

async fn login_page(jar: CookieJar,) -> Result<impl IntoResponse, StatusCode> {
    let token = jar.get("token_key");
    if !token.is_none() {
        let token_val = token.unwrap().value().as_bytes().to_vec();
        let _db_token = home_menu::authorize_token(token_val).await?;
        return Ok((jar, Redirect::to("/")).into_response());
    }
    if  let Some(username) = jar.get("username") && 
        let Some(password) = jar.get("password")
    {
        let username_val = username.value().to_string();
        let password_val = password.value().to_string();
        
        let mut connection = connect_to_db()?;
        let users = home_menu::get_all_users(&mut connection)?;

        for user in users {
            if user.username == username_val && user.password_hash == get_password_hash(&password_val){
                let new_token = home_menu::create_token_for_user(&user, &mut connection).await?;
                let token_value = str::from_utf8(new_token.token.as_slice())
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                    .to_string();
                return Ok((jar.add(Cookie::new("token_key", token_value)), Redirect::to("/")).into_response());
            }
        }
    }
    Ok((jar, get_html("static/login").await).into_response())
}