use handlebars::{RenderError, TemplateError, TemplateRenderError};
use std::io;
use git2;
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
    /// Git interaction error
    Git(git2::Error)
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::DefaultsNotFound => "Could not find default.env file",
            Error::Io(ref e) => e.description(),
            Error::Render(ref e) => e.description(),
            Error::Template(ref e) => e.description(),
            Error::TemplateRender(ref e) => e.description(),
            Error::Homeless => "Could not resolve home directory",
            Error::Git(ref e) => e.description()

        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Render(ref e) => Some(e),
            Error::Template(ref e) => Some(e),
            Error::TemplateRender(ref e) => Some(e),
            Error::Git(ref e) => Some(e),
            _ => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Error {
        Error::Git(error)
    }
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
