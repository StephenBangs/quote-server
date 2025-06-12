// This code originally borrowed from the leptos crate
// examples, where variants appear throughout.
//Framework taken from Bart Massey

use leptos::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Quote {
    pub id: String,
    pub qtext: String,
    pub author: String,
    pub creator: String,
}


pub async fn fetch(endpoint: String) -> Result<Quote, Error> {
    use reqwasm::http::Request;

    let ep = format!(
        //TODO
        "http://localhost:3000/api/{}",
        endpoint,
    );
    let result = Request::get(&ep)
        .send()
        .await?
        // convert it to JSON
        .json()
        .await?;
    Ok(result)
}
