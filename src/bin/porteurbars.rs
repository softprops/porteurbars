#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate porteurbars;
extern crate tempdir;

use clap::{App, Arg, ArgMatches};
use porteurbars::{Result, Template};
use porteurbars::git;
use tempdir::TempDir;

fn run(args: ArgMatches) -> Result<()> {
    let repo = args.value_of("repository").unwrap();
    let url = porteurbars::git::Url::from_str(repo)?;
    let target = args.value_of("target").unwrap_or(".");
    let root = args.value_of("template_root");
    let revision = args.value_of("rev").unwrap_or("master");
    let yes = args.occurrences_of("yes") > 0;
    let replace = args.occurrences_of("keep") > 0;
    info!("Cloning...");
    let tmp = TempDir::new("porteurbars")?;
    git::clone(url, &tmp, revision)?;
    info!("Applying template...");
    Template::new(&tmp).apply(target, root, yes, replace)?;
    println!("off you go");
    Ok(())
}

fn main() {
    env_logger::init();
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("portable git hosted project templates")
        .arg(
            Arg::with_name("repository")
                .value_name("repository")
                .required(true)
                .help(
                    "uri of template to apply.
example uris
github: user/repo
 local: file:///path/to/repo
   git: git@github.com:user/repo.git",
                ),
        )
        .arg(Arg::with_name("target").value_name("target").help(
            "directory to write template output to. defaults to current working directory",
        ))
        .arg(
            Arg::with_name("base")
                .short("b")
                .long("base")
                .value_name("base_directory")
                .takes_value(true)
                .help(
                    "directory within <repository> to use as root. defaults to base of repo",
                ),
        )
        .arg(
            Arg::with_name("rev")
                .short("r")
                .long("rev")
                .value_name("revision")
                .takes_value(true)
                .help("git revision to checkout. defaults to 'master'"),
        )
        .arg(
            Arg::with_name("yes")
                .short("y")
                .long("yes")
                .takes_value(false)
                .help("disables value prompts by accepting all default values"),
        )
        .arg(
            Arg::with_name("keep")
                .short("k")
                .long("keep")
                .takes_value(false)
                .help(
                    "disables replacement prompts and keeps local copies of files",
                ),
        )
        .get_matches();


    if let Err(ref e) = run(args) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";
        writeln!(stderr, "error: {}", e).expect(errmsg);
        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }
        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
