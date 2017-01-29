use handlebars::{RenderError, TemplateError, TemplateRenderError};
use std::io;
// use hyper::Error as HyperError;
// use hyper::status::StatusCode;

/// Enumeration of types of errors
#[derive(Debug)]
pub enum Error {
    /// No default.env file could be found
    DefaultsNotFound,
    /// IO error occurred
    Io(io::Error),
    /// Handlebars render error
    Render(RenderError),
    /// Handlebars template compiler error
    Template(TemplateError),
    /// Handlebars template render error
    TemplateRender(TemplateRenderError),
    /// Home directory could not be resolved
    Homeless,
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

impl From<TemplateRenderError> for Error {
    fn from(error: TemplateRenderError) -> Error {
        Error::TemplateRender(error)
    }
}
