//Main source code to display quote
//6/10/25
//final project submission

//TODO remove - temporary suppressing unused import messages with clippy
#![allow(unused_imports)]

use axum::{routing::{get, post, delete}, Router};
use axum::{
    self,
    RequestPartsExt,
    extract::{Path, Query, State, Json},
    http::{self, StatusCode},
    response::{self, IntoResponse},
    routing,
};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

//milestone 2 additions
use sqlx::sqlite::{ SqlitePool };

//Utoipa doc
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

//Final Submission additions
use utoipa::{ToSchema};
use tower_http::services::ServeFile;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_rapidoc::RapiDoc;
use crate::api::ApiDoc;
use tower_http::{services, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::{net, signal, sync::RwLock, time::Duration};
use chrono::{prelude::*, TimeDelta};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use tower_http::cors::{CorsLayer, Any};
use std::borrow::Cow;
use std::sync::Arc;

//my files
mod quote;
mod api;
mod templates;
mod web;
mod error;

use crate::api::*;
use crate::web::quote_homepage;
use crate::error::AppError;
use crate::quote::{ load_quotes_from_json, Quote}; // json import helper

//clap for cli args
use clap::Parser;

#[derive(Parser)]
pub struct Config {
    //sqlite db uri
    #[arg(long, env = "DATABASE_URL", default_value = "sqlite://db/quotes.db")]
    db_uri: String,

    #[arg(long)]
    init_from: Option<String>
}

//Credit to Bart Massey and Gemini for this code. Using directly from knock-knock-2
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to create SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C (SIGINT) signal.");
        },
        _ = terminate => {
            tracing::info!("Received SIGTERM signal.");
        },
    }

    tracing::info!("Initiating graceful shutdown...");

    // Example: Give some time for in-flight requests to complete
    tokio::time::sleep(Duration::from_secs(2)).await;
    tracing::info!("Cleanup complete.");
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

    //inspiration taken from Knock-Knock-2 by Bart Massey
    //https://github.com/pdx-cs-rust-web/knock-knock-2/
    //build the openapi router
    let (api_router, openapi) =
        OpenApiRouter::with_openapi(ApiDoc::openapi())
            .nest("/api", api::router().into())
            .split_for_parts();

    // swagger UI, Redoc, RapiDoc
    let swagger_ui = SwaggerUi::new("/swagger-ui")
        .url("/api-doc/openapi.json", openapi.clone());
    let redoc_ui = Redoc::with_url("/redoc", openapi.clone());
    let rapidoc_ui = RapiDoc::new("/api-doc/openapi.json")
        .path("/rapidoc"); 
  
    //create main router, mount swagger router
    //shared db pool as state
    //define REST endpoints
    let app = Router::new()
    
        .route("/", get(quote_homepage))
        // Serve CSS from assets/static/quote.css
        .route_service("/static/quote.css", ServeFile::new("assets/static/quote.css"),)
        .merge(swagger_ui)
        .merge(redoc_ui)
        .merge(rapidoc_ui)
        .merge(api_router)
        
        //CORS layer for trunk 
        .layer(
            CorsLayer::new()
            // allow any host (dev only)
            .allow_origin(Any)            
            // GET, POST, etc
            .allow_methods(Any)           
            .allow_headers(Any)
        )

        //HTML ui  
        .with_state(pool);

    //Basic format taken from class example: https://github.com/pdx-cs-rust-web/webhello/blob/axum/src/main.rs
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    eprintln!("quote-server serving http://{}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await.unwrap();
}
