use super::{Error, Result};

use tempdir::TempDir;
use handlebars::{Context, Handlebars, Helper, RenderContext, RenderError};
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File, create_dir_all};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::process::{self, Stdio, Command};
use std::io::{self, Read, Write};

/// file to clone template to
const TMP_PREFIX: &'static str = "porteurbars";

/// subdirectory containing template source
const TEMPLATE_DIR: &'static str = "template";

/// name of file containing key/value pairs representing template defaults
const DEFAULTS: &'static str = "default.env";

pub struct Template {
    /// path to defaults file
    pub defaults: PathBuf,
    /// path to template source
    pub path: PathBuf,
    // holding ref
    _tmp: TempDir,
}

impl Template {
    /// resolve template
    pub fn get(repo: &str) -> Result<Template> {
        let scratch = try!(TempDir::new(TMP_PREFIX));
        try!(clone(repo, scratch.path().to_str().unwrap(), None));
        match find(scratch.path(), DEFAULTS) {
            Ok(Some(defaults)) => {
                Ok(Template {
                    defaults: defaults,
                    path: scratch.path().join(TEMPLATE_DIR),
                    _tmp: scratch,
                })
            }
            _ => Err(Error::DefaultsNotFound),
        }
    }

    /// Apply template
    pub fn apply(&self, target: &Path) -> Result<()> {
        // resolve context
        let map = try!(parse_defaults(self.defaults.as_path()));
        let resolved = try!(interact(&map));
        let data = Context::wraps(&resolved);

        // apply handlebars processing
        let apply = |path: &Path, hbs: &mut Handlebars| -> Result<()> {
            // /tmp/download_dir/templates
            let scratchpath = &format!("{}{}", self.path.to_str().unwrap(), MAIN_SEPARATOR)[..];

            // path relatived based on scratch dir
            let localpath = path.to_str()
                .unwrap()
                .trim_left_matches(scratchpath);

            // eval path as template
            let evalpath = try!(hbs.template_render(&localpath, &resolved));

            // rewritten path, based on target dir and eval path
            let targetpath = target.join(evalpath);

            if path.is_file() {
                let mut file = try!(File::open(path));
                let mut s = String::new();
                try!(file.read_to_string(&mut s));
                let mut file = try!(File::create(targetpath));
                try!(hbs.template_renderw(&s, &data, &mut file));
            } else {
                try!(fs::create_dir_all(targetpath))
            }
            Ok(())
        };

        try!(create_dir_all(target));
        let mut hbs = bars();

        walk(&mut hbs, &self.path, &apply, false)
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
                                            -> ::std::result::Result<(), RenderError> {
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
