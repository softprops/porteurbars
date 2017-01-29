#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate porteurbars;
extern crate tempdir;

use tempdir::TempDir;
use clap::{ArgMatches, App};
use porteurbars::{Result, Template};
use porteurbars::git;

fn run(args: ArgMatches) -> Result<()> {
    let repo = args.value_of("repo").unwrap();
    let url = porteurbars::git::Url::from_str(repo).unwrap();
    let target = args.value_of("target").unwrap_or(".");
    info!("Cloning...");
    let tmp = TempDir::new("porteurbars")?;
    git::clone(url, &tmp, "master");
    info!("Applying template...");
    let template = Template::new(&tmp);
    template.apply(target)?;
    println!("off you go");
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("portable git hosted project templates")
        .about("applies provided data to template")
        .args_from_usage("<repo> 'uri of template to apply'
                          [target] \
                          'directory to write template output to. defaults to current working \
                          directory'")
        .get_matches();


    match run(args) {
        Err(e) => println!("error: {:?}", e),
        _ => (),
    };
}
