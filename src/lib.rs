//! Porteurbars is a tool for sharing portable git hosted project templates

extern crate case;
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
#[macro_use]
extern crate error_chain;

mod defaults;
pub mod git;

mod errors;
pub use errors::{Error, Result, ResultExt};

mod template;
pub use template::Template;
