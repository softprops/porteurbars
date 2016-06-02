use handlebars::{RenderError, TemplateError, TemplateRenderError};
use std::io;

#[derive(Debug)]
pub enum Error {
    DefaultsNotFound,
    Io(io::Error),
    Render(RenderError),
    Template(TemplateError),
    TemplateRender(TemplateRenderError)
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
