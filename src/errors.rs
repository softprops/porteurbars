use handlebars::{RenderError, TemplateError, TemplateRenderError};
use std::io;
use git2;


/// Enumeration of types of errors
error_chain! {

    foreign_links {
        Io(io::Error);
        Render(RenderError);
        Template(TemplateError);
        TemplateRender(TemplateRenderError);
        Git(git2::Error);
    }

    errors {
        DefaultsNotFound {
            description("defaults not found")
            display("defaults not found")
        }
        Homeless {
            description("home directory not defined")
            display("home directory not defined")
        }
    }
}
