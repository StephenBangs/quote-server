//Quote API file.

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
    http::StatusCode,
    Extension,
};
use sqlx::SqlitePool;
use utoipa::{ OpenApi, ToSchema };
use serde::{Deserialize, Serialize};

use crate::quote::Quote;
use crate::error::AppError;

//Final Submission Changes
// OpenAPI document
#[derive(OpenApi)]
#[openapi(
   servers(
        (url = "/api", description = "Quote API base path")
    ), 
    paths(
        get_all_quotes,
        add_quote,
        get_random_quote,
        get_quote_by_id,
        get_quotes_by_author,
        delete_quote
    ),
    components(schemas(Quote)),
    tags(
        (name = "quotes", description = "Quote management endpoints")
    )
)]
pub struct ApiDoc;


// build API router under /api
pub fn router() -> Router<SqlitePool> {
    Router::new()
        .route("/quotes",         get(get_all_quotes).post(add_quote))
        .route("/quotes/random",  get(get_random_quote))
        .route("/quotes/{id}",    get(get_quote_by_id).delete(delete_quote))
        .route("/quotes/author/{author}", get(get_quotes_by_author))
}

/// GET /quotes - list all quotes in json
#[utoipa::path(
    get,
    path = "/quotes",
    responses(
        (status = 200, description = "List all quotes", body = [Quote]),
        (status = 500, description = "Internal error")
    )
)]
pub async fn get_all_quotes(
    State(pool): State<SqlitePool>
) -> Result<Json<Vec<Quote>>, AppError> {
    let quotes = sqlx::query_as!(Quote, "SELECT * FROM quotes")
        .fetch_all(&pool)
        .await?;
    Ok(Json(quotes))
}

// struct used for inserting new quotes via POST
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct NewQuote {
    pub id: String,
    pub qtext: String,
    pub author: String,
    pub creator: String,
}

// POST /quotes add quote
#[utoipa::path(
    post,
    path = "/quotes",
    request_body = NewQuote,
    responses(
        (status = 200, description = "Quote added", body = Quote),
        (status = 500, description = "Internal error")
    )
)]
pub async fn add_quote(
    State(pool): State<SqlitePool>,
    Json(new): Json<NewQuote>,
) -> Result<Json<Quote>, AppError> {
    let inserted = sqlx::query_as!(
        Quote,
        r#"INSERT INTO quotes (id, qtext, author, creator)
           VALUES (?, ?, ?, ?)
           RETURNING id, qtext, author, creator"#,
        new.id, new.qtext, new.author, new.creator
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(inserted))
}

// GET /quotes/random - get random quote
#[utoipa::path(
    get,
    path = "/quotes/random",
    responses(
        (status = 200, description = "Random quote", body = Quote),
        (status = 404, description = "No quotes available")
    )
)]
pub async fn get_random_quote(
    State(pool): State<SqlitePool>
) -> Result<Json<Quote>, AppError> {
    let quote = sqlx::query_as!(
        Quote,
        "SELECT * FROM quotes ORDER BY RANDOM() LIMIT 1"
    )
    .fetch_optional(&pool)
    .await?;
    quote
        .map(Json)
        .ok_or(AppError::NotFound)
}

// GET /quotes/{id} - get quote by searching for ID
#[utoipa::path(
    get,
    path = "/quotes/{id}",
    params(
        ("id" = String, Path, description = "Quote ID"),
    ),
    responses(
        (status = 200, description = "Quote by ID", body = Quote),
        (status = 404, description = "Quote not found")
    )
)]
pub async fn get_quote_by_id(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Quote>, AppError> {
    let quote = sqlx::query_as!(
        Quote,
        "SELECT * FROM quotes WHERE id = ?",
        id
    )
    .fetch_optional(&pool)
    .await?;
    quote
        .map(Json)
        .ok_or(AppError::NotFound)
}

// GET /quotes/author/{author} - get quote by searching by author
#[utoipa::path(
    get,
    path = "/quotes/author/{author}",
    params(
        ("author" = String, Path, description = "Author name"),
    ),
    responses(
        (status = 200, description = "Quotes by author", body = [Quote]),
        (status = 500, description = "Internal error")
    )
)]
pub async fn get_quotes_by_author(
    State(pool): State<SqlitePool>,
    Path(author): Path<String>,
) -> Result<Json<Vec<Quote>>, AppError> {
    let quotes = sqlx::query_as!(
        Quote,
        "SELECT * FROM quotes WHERE author = ?",
        author
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(quotes))
}

// DELETE /quotes/{id} - delete a quote from database by ID
#[utoipa::path(
    delete,
    path = "/quotes/{id}",
    params(
        ("id" = String, Path, description = "Quote ID"),
    ),
    responses(
        (status = 200, description = "Quote deleted"),
        (status = 404, description = "Quote not found")
    )
)]
pub async fn delete_quote(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM quotes WHERE id = ?", id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(())
    }
}