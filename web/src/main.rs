use axum::{response::Html, routing::get, Router};
use minijinja::{path_loader, Environment};
use std::collections::HashMap;

async fn index() -> Result<Html<String>, Html<String>> {
    // Create a Minijinja environment and set up a loader
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    // Load and parse the HTML template
    let template = env
        .get_template("index.html")
        .map_err(|_| Html("Internal Server Error".to_string()))?;

    // Define dynamic data to render in the template
    let mut context = HashMap::new();
    context.insert("title", "My Page Title");
    context.insert("name", "Dax");

    // Render the template with the dynamic data
    let rendered = template
        .render(&context)
        .map_err(|_| Html("Internal Server Error".to_string()))?;

    Ok(Html(rendered))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
