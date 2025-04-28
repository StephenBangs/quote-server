//HTML template struct for now. 

use askama::Template;
use crate::quote::Quote;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub quote: &'a Quote,
}
