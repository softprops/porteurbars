extern crate handlebars;
extern crate hyper;
extern crate tempdir;
extern crate regex;
extern crate tar;
extern crate flate2;

mod errors;
pub use errors::Error;

mod template;
pub use template::{Template, templates_dir};

pub type Result<T> = std::result::Result<T, Error>;
