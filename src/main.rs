//Main source code to display quote

mod quote;
mod templates;

use axum::{routing::get, Router, response::Html};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use templates::IndexTemplate;
use askama::Template;
//TODO:
//testing if this lets me compile
//use hyper::Server;

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

    //TODO:

    // //define listening addr
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("listening on http://{}", addr);

    //New swap to include a hyper Server, since I don't know how to do this 
    //otherwise right now
    //Server::bind(&addr)

    //start server
    //Not working
    //axum::Server::bind(&addr)
    
    //Per cargo clippy recommendation, no idea? 
    //This seems absolutely not relevant
    //tokio::net::windows::named_pipe::PipeEnd::bind(&addr)

    // Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

async fn show_quote() -> Html<String> {

    let quote = quote::Quote {
        text: "For a time, I rest in the grace of the world, and am free.".to_string(),
        author: "Wendell Berry".to_string(),
    };
    
    //create template for quote, then render
    let template = IndexTemplate { quote: &quote };
    Html(template.render().unwrap())
}
