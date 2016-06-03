extern crate clap;
extern crate porteurbars;

use clap::{App, SubCommand};
use porteurbars::Template;
use std::path::Path;


fn main() {

    let args = App::new("pb")
        .version(env!("CARGO_PKG_VERSION"))
        .about("portable git hosted project templates")
        .subcommand(SubCommand::with_name("apply")
                    // apply -t foo/bar .
            .about("applies provided data to template")
            .args_from_usage("-t, --template=<template> 'uri of template to apply'
                              [target] 'directory to write template output to'"))
        .get_matches();

    if let Some(args) = args.subcommand_matches("apply") {
        // todo extract resolver

        match Template::get(args.value_of("template").unwrap()) {
            Ok(tmpl) => {
                tmpl.apply(Path::new(args.value_of("target").unwrap_or("."))).unwrap();
            },
            _ => ()
        }

        println!("off you go")
    }


}
