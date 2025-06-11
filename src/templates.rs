//HTML template struct for now. 



//TODO
//previously working
/* #[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub quote: &'a Quote,
}
*/
use askama::Template;
use crate::quote::Quote;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub quote: Quote,
}
