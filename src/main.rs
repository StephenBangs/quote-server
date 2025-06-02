//Main source code to display quote
//5/30/25
//Second check in

use axum::{routing::{get, post, delete}, Router, response::Html, debug_handler};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use templates::IndexTemplate;
use askama::Template;
//milestone 2 additions
use sqlx::sqlite::{ SqlitePoolOptions, SqlitePool };
use std::env;
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
use crate::quote::{ load_quotes_from_json, Quote, ImportQuote, }; // json import helper

//TODO clap
use clap::Parser;

//swagger
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

#[tokio::main]
async fn main() {
    println!("\nStarting Quote Server.\n"); 

    let config = Config::parse();

    let pool = SqlitePool::connect(&config.db_uri)
        .await
        .expect("Failed to connect to database");

    //if cli flag --init-from is passed, load from json file.
    if let Some(json_path) = &config.init_from {
        load_quotes_from_json(&pool, json_path).await;
    }
   
    // //7
    // let swagger_router: Router<()> = SwaggerUi::new("/swagger-ui")
    //     .url("/api-doc/openapi.json", ApiDoc::openapi())
    //     .into(); 
    // //7 
    // let swagger_with_state: Router<SqlitePool> = swagger_router.map_state(|()| pool.clone()); 

    //10
    // let swagger_subrouter = SwaggerUi::new("/swagger-ui")
    //     .url("/api-doc/openapi.json", ApiDoc::openapi())
    //     .build();
    // let app_without_state = Router::new()
    //     .merge(swagger_subrouter);
    // let app_with_state = app_without_state.with_state(pool.clone());


    let swagger_router = SwaggerUi::new("/swagger-ui")
        .url("/api-doc/openapi.json", ApiDoc::openapi());
   
    //app router
    //8-9
    //let app = Router::new()        
    //10
    //let app = app_with_state
    let app = Router::new()
        .merge(swagger_router)

        .with_state(pool.clone())

        //REST api endpoints, hopefully
        .route("/api/quotes", post(add_quote))
        .route("/api/quotes/:id", delete(delete_quote))
        .route("/api/quotes/random", get(get_random_quote))
        .route("/api/quotes/author/:author", get(get_quotes_by_author))  
        .route("/", get(quote_homepage))
        
        .with_state(pool); 
        //mount swagger ui my merging in own router

        //8-9
        // .route_service(
        //     "/swagger-ui",
        //     SwaggerUi::new("/swagger-ui")
        //         .url("/api-doc/openapi.json", ApiDoc::openapi())
        // )
        
        //8
        //.merge(swagger_with_state)
        //7
/*         .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-doc/openapi.json", ApiDoc::openapi())
                .into_router()
                .with_state(pool.clone())
        ) */

    //Basic format taken from class example: https://github.com/pdx-cs-rust-web/webhello/blob/axum/src/main.rs
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    eprintln!("quote-server serving http://{}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//hardcoded quote currently
async fn show_quote() -> Html<String> {
    let quote = Quote {
        id: "01".to_string(),
        qtext: "For a time, I rest in the grace of the world, and am free.".to_string(),
        author: "Wendell Berry".to_string(),
        creator: "Admin".to_string(),
};
    
    //create template for quote, then render
    let template = IndexTemplate { quote };
    Html(template.render().unwrap())
}