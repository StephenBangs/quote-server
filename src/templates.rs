//HTML template struct for now. 

use askama::Template;
use askama_axum::IntoResponse;
use crate::quote::Quote;

//TODO
//previously working
/* #[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub quote: &'a Quote,
}
*/

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub quote: Quote,
}
