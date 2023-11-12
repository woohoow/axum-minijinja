use argh::FromArgs;
use axum::{response::Html, routing::get, Router};
use csv::Reader;
use minijinja::{path_loader, Environment};
use std::collections::HashMap;
use std::fs::File;

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

fn read_csv_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        // Process CSV record
        println!("{:?}", record);
    }

    Ok(())
}

#[derive(FromArgs)]
/// Axum Web Server with CSV Parser
struct Args {
    #[argh(option, short = 'c', long = "csv")]
    /// CSV file to parse
    csv_file: Option<String>,

    #[argh(switch, short = 'r', long = "runserver")]
    /// run the Axum server
    run_server: bool,
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    if let Some(csv_file) = args.csv_file {
        if let Err(err) = read_csv_file(&csv_file) {
            eprintln!("Error parsing CSV file: {:?}", err);
        }
    }

    if args.run_server {
        let app = Router::new().route("/", get(index));

        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
