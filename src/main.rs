//Main source code to display quote
//5/30/25
//Second check in

mod quote;
mod templates;

use axum::{routing::{get, post, delete}, Router, response::Html};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use templates::IndexTemplate;
use askama::Template;
//milestone 2 additions
use crate::quote::load_quotes_from_json;
use crate::web::quote_homepage;
//Utoipa doc
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::quote::Quote;        // data model
use crate::quote_api::{         // route handlers documented
    add_quote, delete_quote,
    get_random_quote, get_quotes_by_author,
};

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

//TODO clap
clap::Parser;

struct Config {

    //sqlite db uri
    #[arg(long, env = "DATABASE_URL", default_value = "sqlite://db/quotes.db")]
    db_uri: String,

    #[arg(long)]
    init_from: Option<String>,

}

#[tokio::main]
async fn main() {
    println!("\nStarting Quote Server.\n");

    //TODO clap?
    //for cli flag
/*     if let Some(json_path) = &config.init_from {
        load_quotes_from_json(&db_pool, json_path).await;
    }
 */

        let app = Router::new()
            .route("/", get(quote_homepage))

            .route("/api/quotes", post(add_quote))
            .route("/api/quotes/:id", delete(delete_quote))
            .route("/api/quotes/random", get(get_random_quote))
            .route("/api/quotes/author/:author", get(get_quotes_by_author))
    
    // swagger UI under /swagger-ui, backed by json
    .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))

    // Add database state so handlers can access the connection pool
    .with_state(db_pool); 

    //Basic format taken from class example: https://github.com/pdx-cs-rust-web/webhello/blob/axum/src/main.rs
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);
    eprintln!("quote-server serving http://{}", ip);
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//hardcoded quote currently
async fn show_quote() -> Html<String> {

    let quote = quote::Quote {
        id: "01".to_string(),
        text: "For a time, I rest in the grace of the world, and am free.".to_string(),
        author: "Wendell Berry".to_string(),
        creator: "Admin".to_string(),
    };
    
    //create template for quote, then render
    let template = IndexTemplate { quote: &quote };
    Html(template.render().unwrap())
}

//cli flag if wanted
#[derive(Parser)]
#[command(author, version, about)]
pub struct Config {
    #[arg(long)]
    pub init_from: Option<String>,
}