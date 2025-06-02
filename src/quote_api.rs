//Quote API file.
//TODO

use axum::{extract::{Path, State}, Json};
use sqlx::SqlitePool;
use utoipa::ToSchema;
use utoipa::openapi::path::{Operation, Parameter};
use utoipa::path;
use serde::{Deserialize, Serialize};

use crate::quote::Quote;
use crate::error::AppError;

// struct used for inserting new quotes via POST
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct NewQuote {
    pub id: String,
    pub qtext: String,
    pub author: String,
    pub creator: String,
}

#[utoipa::path(
    post,
    path = "/api/quotes",
    request_body = NewQuote,
    responses(
        (status = 200, description = "Quote added", body = Quote),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_quote( State(pool): State<SqlitePool>, Json(new): Json<NewQuote>, ) -> Result<Json<Quote>, AppError> {
    let inserted = sqlx::query_as!(
        Quote,
        r#"INSERT INTO quotes (id, qtext, author, creator)
           VALUES (?, ?, ?, ?)
           RETURNING id, qtext, author, creator"#,
        new.id,
        new.qtext,
        new.author,
        new.creator,
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(inserted))
}

#[utoipa::path(
    delete,
    path = "/api/quotes/{id}",
    params(
        ("id" = String, Path, description = "ID of the quote to delete")
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
    let rows = sqlx::query!("DELETE FROM quotes WHERE id = ?", id)
        .execute(&pool)
        .await?
        .rows_affected();

    if rows == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(())
    }
}

#[utoipa::path(
    get,
    path = "/api/quotes/random",
    responses(
        (status = 200, description = "Random quote", body = Quote),
        (status = 404, description = "No quotes in database")
    )
)]
pub async fn get_random_quote(State(pool): State<SqlitePool>,) -> Result<Json<Quote>, AppError> {
    let quote = sqlx::query_as!(
        Quote,
        r#"SELECT * FROM quotes ORDER BY RANDOM() LIMIT 1"#
    )
    .fetch_optional(&pool)
    .await?;

    match quote {
        Some(q) => Ok(Json(q)),
        None => Err(AppError::NotFound),
    }
}

#[utoipa::path(
    get,
    path = "/api/quotes/author/{author}",
    params(
        ("author" = String, Path, description = "Author name to search for")
    ),
    responses(
        (status = 200, description = "Quotes by author", body = [Quote])
    )
)]
pub async fn get_quotes_by_author(
    State(pool): State<SqlitePool>,
    Path(author): Path<String>,
) -> Result<Json<Vec<Quote>>, AppError> {
    let quotes = sqlx::query_as!(
        Quote,
        r#"SELECT * FROM quotes WHERE author = ?"#,
        author
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(quotes))
}