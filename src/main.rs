//Main source code to display quote
//6/10/25
//final project submission

use axum::{routing::{get, post, delete}, Router};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

//milestone 2 additions
use sqlx::sqlite::{ SqlitePool };

//Utoipa doc
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod quote;
mod quote_api;
mod templates;
mod web;
mod error;

use crate::quote_api::*;
use crate::web::quote_homepage;
use crate::error::AppError;
use crate::quote::{ load_quotes_from_json, Quote}; // json import helper

//clap for cli args
use clap::Parser;

//swagger ui definitions for openapi. Generated a baseline idea of how to do this with chatGPT
#[derive(OpenApi)]
#[openapi(
    paths(
        add_quote,
        delete_quote,
        get_random_quote,
        get_quotes_by_author
    ),
    components(schemas(Quote)),
    tags(
        (name = "quotes", description = "Famous quote API endpoints")
    )
)]
pub struct ApiDoc;

#[derive(Parser)]
pub struct Config {
    //sqlite db uri
    #[arg(long, env = "DATABASE_URL", default_value = "sqlite://db/quotes.db")]
    db_uri: String,

    #[arg(long)]
    init_from: Option<String>
}

//Main can set --init-from to db folder
#[tokio::main]
async fn main() {
    println!("\nStarting Quote Server.\n"); 

    //parse cmd line args
    let config = Config::parse();

    let pool = SqlitePool::connect(&config.db_uri)
        .await
        .expect("Failed to connect to database");

    //Adding auto migration for cargo sqlx migrate run
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    //if cli flag --init-from is passed, load from json file.
    if let Some(json_path) = &config.init_from {
        let _ = load_quotes_from_json(&pool, json_path)
        .await
        .expect("Failed to import quotes from json");
    }
   
    //building swagger router with openapi doc
    let swagger_router = SwaggerUi::new("/swagger-ui")
        .url("/api-doc/openapi.json", ApiDoc::openapi());
  
    //create main router, mount swagger router
    //shared db pool as state
    //define REST endpoints
    let app = Router::new()
        .merge(swagger_router)
        .with_state(pool.clone())

        //REST api endpoints, hopefully
        .route("/api/quotes", post(add_quote))
        .route("/api/quotes/{id}", delete(delete_quote))
        .route("/api/quotes/random", get(get_random_quote))
        .route("/api/quotes/author/{author}", get(get_quotes_by_author))
        
        //HTML ui  
        .route("/", get(quote_homepage))
        //TODO for some reason it wants this as well
        .with_state(pool);

    
    //Basic format taken from class example: https://github.com/pdx-cs-rust-web/webhello/blob/axum/src/main.rs
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    eprintln!("quote-server serving http://{}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    //axum::serve(listener, app).await.unwrap();
    
    //TODO test 2 
    axum::serve(listener, app).await.unwrap();
}

//hardcoded quote currently
/* async fn show_quote() -> Html<String> {
    let quote = Quote {
        id: "01".to_string(),
        qtext: "For a time, I rest in the grace of the world, and am free.".to_string(),
        author: "Wendell Berry".to_string(),
        creator: "Admin".to_string(),
};
    
    //create template for quote, then render
    let template = IndexTemplate { quote };
    Html(template.render().unwrap())
} */