//HTML template struct for now. 

use askama::Template;
use crate::quote::Quote;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub quote: Quote,
}
