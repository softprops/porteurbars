#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate difference;
extern crate handlebars;
extern crate tempdir;
extern crate regex;
extern crate walkdir;
extern crate git2;

mod defaults;
pub mod git;

mod errors;
pub use errors::Error;

mod template;
pub use template::Template;

pub type Result<T> = std::result::Result<T, Error>;
