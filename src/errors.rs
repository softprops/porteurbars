use handlebars::{RenderError, TemplateError};
use std::io;

#[derive(Debug)]
pub enum Error {
    DefaultsNotFound,
    Io(io::Error),
    Render(RenderError),
    Template(TemplateError)
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<RenderError> for Error {
    fn from(error: RenderError) -> Error {
        Error::Render(error)
    }
}

impl From<TemplateError> for Error {
    fn from(error: TemplateError) -> Error {
        Error::Template(error)
    }
}
