extern crate handlebars;
extern crate tempdir;

pub mod errors;
pub use errors::Error;

use handlebars::{Context, Handlebars, Helper, RenderContext, RenderError};
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File, create_dir_all};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::process::{self, Stdio, Command};
use std::io::{self, Read, Write};

const TMP_PREFIX: &'static str = "porteurbars";

pub type Result<T> = std::result::Result<T, Error>;

pub struct Template<'a> {
    /// target output dir
    pub target: &'a Path,
    /// defaults env faile
    pub defaults: &'a str,
    /// relative path to project template src
    pub project: &'a Path,
    /// remote git repo
    pub repo: &'a str,
}

impl<'a> Template<'a> {
    /// Apply project template
    pub fn apply(&self) -> Result<&'a Path> {
        // source files
        // expected layout
        // defaults.env
        // template/*
        let scratch = try!(tempdir::TempDir::new(TMP_PREFIX));
        try!(clone(self.repo, scratch.path().to_str().unwrap(), None));

        // exclusion rules
        let exclude = |path: &str| -> bool { path.starts_with(".git") };

        match find(scratch.path(), self.defaults) {
            Ok(Some(defaults)) => {

                // resolve context
                let map = try!(parse_defaults(defaults.as_path()));
                let resolved = try!(interact(&map));
                let data = Context::wraps(&resolved);

                // apply handlebars processing
                let apply = |path: &Path, hbs: &mut Handlebars| -> Result<()> {
                    // /tmp/download_dir/
                    let scratchpath =
                        &format!("{}{}", scratch.path().to_str().unwrap(), MAIN_SEPARATOR)[..];

                    // path relatived based on scratch dir
                    let localpath = path.to_str()
                        .unwrap()
                        .trim_left_matches(scratchpath);

                    // eval path as template
                    let evalpath = try!(hbs.template_render(&localpath, &resolved));

                    // rewritten path, based on target dir and eval path
                    let targetpath = self.target.join(evalpath);

                    if !exclude(localpath) {
                        if path.is_file() {
                            let mut file = try!(File::open(path));
                            let mut s = String::new();
                            try!(file.read_to_string(&mut s));
                            let mut file = try!(File::create(targetpath));
                            try!(hbs.template_renderw(&s, &data, &mut file));
                        } else {
                            try!(fs::create_dir_all(targetpath))
                        }
                    }
                    Ok(())
                };

                try!(create_dir_all(self.target));
                let mut hbs = bars();
                try!(walk(&mut hbs, scratch.path(), &apply, false));

                Ok(self.target)
            }
            _ => Err(Error::DefaultsNotFound),
        }
    }
}

pub fn bars() -> Handlebars {
    let mut hbs = Handlebars::new();
    fn transform<F>(bars: &mut Handlebars, name: &str, f: F)
        where F: 'static + Fn(&str) -> String + Sync + Send
    {
        bars.register_helper(name,
                             Box::new(move |c: &Context,
                                            h: &Helper,
                                            _: &Handlebars,
                                            rc: &mut RenderContext|
                                            -> std::result::Result<(), RenderError> {
                                 let param = h.params().get(0).unwrap();
                                 let value = c.navigate(rc.get_path(), param);
                                 try!(rc.writer.write(f(value.as_string().unwrap()).as_bytes()));
                                 Ok(())
                             }));
    }

    transform(&mut hbs, "upper", str::to_uppercase);
    transform(&mut hbs, "lower", str::to_lowercase);

    hbs
}

/// prompt for a value defaulting to a given string when an answer is not available
fn prompt(name: &str, default: &str) -> io::Result<String> {
    let mut answer = String::new();
    print!("{} [{}]: ", name, default);
    try!(io::stdout().flush());
    try!(io::stdin().read_line(&mut answer));
    let trimmed = answer.trim();
    if trimmed.trim().is_empty() {
        Ok(default.to_owned())
    } else {
        Ok(trimmed.to_owned())
    }
}

/// clone a repository to a given location
fn clone(repo: &str, to: &str, branch: Option<&str>) -> io::Result<process::Output> {
    let mut git = Command::new("git");
    git.arg("clone");

    if let Some(b) = branch {
        git.arg(format!("-b {}", b));
        git.arg("--single-branch");
    };

    git.arg(repo)
        .arg(to)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

fn parse_defaults(p: &Path) -> io::Result<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();
    let mut f = try!(File::open(p));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    let values = s.lines()
        .map(|l| l.split("=").take(2).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    for pair in values.iter() {
        if pair.len() == 2 {
            map.insert(pair[0].trim().to_owned(), pair[1].trim().to_owned());
        }
    }

    Ok(map)
}

/// given a set of defaults, attempt to interact with a user
/// to resolve the parameters that can not be inferred
fn interact(defaults: &BTreeMap<String, String>) -> io::Result<BTreeMap<String, String>> {
    let mut resolved = BTreeMap::new();
    for (k, v) in defaults {
        let answer = match env::var(k) {
            Ok(v) => v,
            _ => try!(prompt(k, v)),
        };
        resolved.insert(k.clone(), answer);
    }
    Ok(resolved)
}

fn walk<F>(hbs: &mut Handlebars, dir: &Path, f: &F, include_dir: bool) -> Result<()>
    where F: Fn(&Path, &mut Handlebars) -> Result<()>
{
    if try!(fs::metadata(dir)).is_dir() {
        if include_dir {
            try!(f(&dir, hbs));
        }
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(walk(hbs, &entry.path(), f, true));
            } else {
                try!(f(&entry.path().as_path(), hbs));
            }
        }
    }
    Ok(())
}

fn find(target_dir: &Path, target_name: &str) -> io::Result<Option<PathBuf>> {
    for entry in try!(fs::read_dir(target_dir)) {
        let e = try!(entry);
        if let Some(name) = e.file_name().to_str() {
            if name == target_name {
                return Ok(Some(e.path()));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;
    #[test]
    fn test_bars() {
        let mut map = BTreeMap::new();
        map.insert("name".to_owned(), "porteurbars".to_owned());
        assert_eq!("Hello, PORTEURBARS",
                   bars().template_render("Hello, {{upper name}}", &map).unwrap());
    }
}
