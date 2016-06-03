extern crate handlebars;
extern crate tempdir;

mod errors;
pub use errors::Error;

mod template;
pub use template::Template;

pub type Result<T> = std::result::Result<T, Error>;
