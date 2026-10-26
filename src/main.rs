use std::{fs::{File, OpenOptions}, io::{BufReader, Read, Write}};

use axum::{
    Router, http::StatusCode, response::Html, routing::{get, post},
};
use home_menu::get_html;
use sha256::digest;


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(main_page))
        .route("/wake", post(wake_gui))
    ;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn main_page() -> Result<Html<String>, StatusCode> {
    let html = get_html("static/test.html").await?;
    Ok(html)
}

async fn wake_gui(body: String) -> StatusCode {
    StatusCode::OK
}

