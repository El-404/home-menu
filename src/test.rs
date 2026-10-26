async fn john_or_404(Query(params): Query<HashMap<String, String>>) -> Result<String, StatusCode> {
    let default = "World".to_string();
    let name = params.get("name").unwrap_or(&default);
    if name != "John" {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(format!("Hello, {}!", name))
}

async fn from_file() -> Result<Html<String>, StatusCode> {
    get_html("static/test.html").await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}