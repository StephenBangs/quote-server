//web handler for askama templating

use axum::{extract::State};
//use axum::debug_handler;
use askama_axum::IntoResponse;
use sqlx::SqlitePool;
use crate::error::AppError;
use crate::templates::IndexTemplate;
use crate::quote::Quote;

// pulls a random quote from the database and passes to Askama template.
//#[debug_handler]
pub async fn quote_homepage(State(pool): State<SqlitePool>,) -> Result<IndexTemplate, AppError> {
    // fetch a random quote from database
    let quote = sqlx::query_as!(Quote, r#"SELECT * FROM quotes ORDER BY RANDOM() LIMIT 1"#)
        .fetch_optional(&pool)
        .await?;

    match quote {
        Some(q) => Ok(IndexTemplate { quote: q }),
        None => Err(AppError::NotFound),
    }
}