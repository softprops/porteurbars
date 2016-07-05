extern crate clap;
extern crate porteurbars;

use clap::{ArgMatches, App, SubCommand};
use porteurbars::{Result, Template, templates_dir};
use std::path::Path;

fn run(args: ArgMatches) -> Result<()> {
    match args.subcommand() {
        ("init", Some(args)) => {
            let force = args.occurrences_of("f") > 0;
            try!(Template::init(force));
            println!("intialized")
        }
        ("validate", Some(args)) => {
            try!(Template::validate(Path::new(args.value_of("template").unwrap())));
            println!("template is valid")
        }
        ("ls", _) => {
            let templates = try!(Template::list());
            for template in templates {
                println!("{}", template);
            }
        }
        ("rm", Some(args)) => {
            let deleted = try!(Template::delete(args.value_of("tag").unwrap()));
            if deleted {
                println!("template deleted")
            } else {
                println!("template does not exist")
            }
        }
        ("get", Some(args)) => {
            let valid = try!(Template::download(args.value_of("template").unwrap(),
                                                args.value_of("tag")));
            if valid {
                println!("downloaded template")
            } else {
                println!("invalid template");
            }
        }
        ("apply", Some(args)) => {
            // todo: download if it doesn't exist?
            let path = templates_dir().unwrap().join(args.value_of("tag").unwrap());
            let tmpl = try!(Template::get(&path));
            tmpl.apply(Path::new(args.value_of("target").unwrap_or(".")))
                .unwrap();
            println!("off you go")
        }
        _ => (),
    };
    Ok(())
}

fn main() {

    let args = App::new("pb")
        .version(env!("CARGO_PKG_VERSION"))
        .about("portable git hosted project templates")
        .subcommand(SubCommand::with_name("init")
            .about("initializes current working directory with porteurbars required contents")
            .args_from_usage("-f, --force 'force content creation event if content already \
                              exists'"))
        .subcommand(SubCommand::with_name("validate")
            .about("validates that a template at given path is valid")
            .args_from_usage("<template> 'path to template'"))
        .subcommand(SubCommand::with_name("get")
            .about("get's a remote template and stores it locally for use")
            .args_from_usage("<template> 'uri of template to fetch'
                              [tag] 'name of the template'"))
        .subcommand(SubCommand::with_name("rm")
            .about("removes a template from local cache")
            .args_from_usage("<tag> 'name of the template'"))
        .subcommand(SubCommand::with_name("ls").about("lists local templates"))
        .subcommand(SubCommand::with_name("apply")
            .about("applies provided data to template")
            .args_from_usage("<tag> 'uri of template to apply'
                              [target] 'directory to write template output to'"))
        .get_matches();

    let _ = run(args).unwrap();
}
