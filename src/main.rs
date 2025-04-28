//Main source code to display quote

mod quote;
mod templates;

use axum::{routing::get, Router, response::Html};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use templates::IndexTemplate;
use askama::Template;

#[tokio::main]
async fn main() {
    println!("\nStarting Quote Server.\n");

    //create new router and route
    let app = Router::new()
        .route("/", get(show_quote));

    //Basic format taken from class example: https://github.com/pdx-cs-rust-web/webhello/blob/axum/src/main.rs
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    eprintln!("quote-server serving http://{}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//hardcoded quote currently
async fn show_quote() -> Html<String> {

    let quote = quote::Quote {
        text: "For a time, I rest in the grace of the world, and am free.".to_string(),
        author: "Wendell Berry".to_string(),
    };
    
    //create template for quote, then render
    let template = IndexTemplate { quote: &quote };
    Html(template.render().unwrap())
}
