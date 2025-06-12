//web handler for askama templating

use axum::extract::State;
use axum::response::Html;
use askama::Template;
use sqlx::SqlitePool;
use crate::error::AppError;
use crate::templates::IndexTemplate;
use crate::quote::Quote;

// get quote homepage randomly
pub async fn quote_homepage(
    State(pool): State<SqlitePool>,
) -> Result<Html<String>, AppError> {
    // Fetch a random quote for initial page load
    let quote = sqlx::query_as!(
        Quote,
        "SELECT * FROM quotes ORDER BY RANDOM() LIMIT 1"
    )
    .fetch_optional(&pool)
    .await?;
    let quote = quote.ok_or(AppError::NotFound)?;
    let html = IndexTemplate { quote }
        .render()
        .map_err(AppError::TemplateError)?;
    Ok(Html(html))
}