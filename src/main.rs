extern crate clap;
extern crate porteurbars;

use clap::App;
use porteurbars::Project;
use std::path::Path;

fn main() {
    let args = App::new("porteurbars")
                   .version(env!("CARGO_PKG_VERSION"))
                   .about("portable git hosted handlebars project templates")
                   .args_from_usage("-t, --target=[target] 'Target output directory. Defaults to current working directory'
                                     -d, --defaults=[defaults] 'Env file to load default names and values from. Defaults to default.env'
                                     -s, --src=[project] 'Project template src base path.Defaults to /src'
                                     <repo> 'git repository uri storing template src in full git uri form or github shorthand (user/repo)'")
                   .get_matches();

    let project = Project {
        target: Path::new(args.value_of("target").unwrap_or(".")),
        defaults: args.value_of("defaults").unwrap_or("default.env"),
        project: Path::new(args.value_of("project").unwrap_or(".")),
        repo: args.value_of("repo").unwrap(),
    };

    project.apply().unwrap();

    println!("off you go")
}
