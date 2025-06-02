//Quote struct file. For storing and managing quotes.

//use serde for (de)serialization
use serde::{Deserialize, Serialize};
//use utoipa for api gen
use utoipa::ToSchema;
//use askama for templating
use askama::Template;
//use sqlx for grabbing quotes
use sqlx::FromRow;

use sqlx::SqlitePool;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use crate::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Quote {
    pub id: String,
    pub qtext: String,
    pub author: String,
    pub creator: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImportQuote {
    pub id: String,
    pub qtext: String,
    pub author: String,
    pub creator: String,
}

pub async fn load_quotes_from_json(pool: &SqlitePool, path: &str,) -> Result<(), AppError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let quotes: Vec<ImportQuote> = serde_json::from_reader(reader)?;

    for quote in quotes {
        sqlx::query!(
            r#"
            INSERT INTO quotes (id, qtext, author, creator)
            VALUES (?, ?, ?, ?)
            "#,
            quote.id,
            quote.qtext,
            quote.author,
            quote.creator,            
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}