use super::{Error, Result};

use difference;
use tempdir::TempDir;
use handlebars::{Context, Handlebars, Helper, RenderContext, RenderError};
use std::collections::BTreeMap;
use std::env;
use tar::Archive;
use std::fs::{self, File, create_dir_all, read_dir, rename, OpenOptions};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::io::{self, Read, Write};
use hyper::Client;
use hyper::header::UserAgent;
use flate2::read::GzDecoder;


/// file to clone template to
// const TMP_PREFIX: &'static str = "porteurbars";
/// subdirectory containing template source
const TEMPLATE_DIR: &'static str = "template";

/// name of file containing key/value pairs representing template defaults
const DEFAULTS: &'static str = "default.env";

pub fn templates_dir() -> Result<PathBuf> {
    let path = try!(env::home_dir().ok_or(Error::Homeless))
        .join(".porteurbars")
        .join("templates");
    Ok(path)
}

/// A template holds a path to template source and a
/// file describing the default values associated with
/// names used in the template
pub struct Template {
    /// path to defaults file
    pub defaults: PathBuf,
    /// path to template source
    pub path: PathBuf,
}

impl Template {
    /// validates a template located at `path`
    pub fn validate(path: &Path) -> Result<bool> {
        if !path.exists() {
            return Ok(false);
        }
        if !path.join(TEMPLATE_DIR).exists() {
            return Ok(false);
        }
        let tmpdir = try!(TempDir::new("pb-test"));
        let template = try!(Template::get(path));
        let defaults = try!(parse_defaults(template.defaults.as_path()));
        for (k, _) in defaults {
            env::set_var(k, "test_value")
        }
        let _ = try!(template.apply(tmpdir.path()));
        Ok(true)
    }

    /// initializes current working directory with porteurbar defaults
    pub fn init(force: bool) -> Result<()> {
        if Path::new(TEMPLATE_DIR).exists() && !force {
            return Ok(());
        }
        try!(fs::create_dir(TEMPLATE_DIR));
        try!(fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(DEFAULTS));
        Ok(())
    }

    /// downloads a template from repo (user/repo)
    /// todo: handle host (ghe), branch, credentials (private repo)
    pub fn download(repo: &str, tag: Option<&str>) -> Result<bool> {
        let template_dir = try!(templates_dir()).join(tag.unwrap_or(&repo.replace("/", "-")[..]));
        if template_dir.exists() {
            return Ok(true);
        }
        let download = try!(TempDir::new("porteurbars-dl"));
        let host = "api.github.com";
        let branch = "master";
        let res = try!(Client::new()
            .get(&format!("https://{}/repos/{}/tarball/{}", host, repo, branch)[..])
            .header(UserAgent("porteurbars/0.1.0".to_owned()))
            .send());
        try!(Archive::new(try!(GzDecoder::new(res))).unpack(download.path()));
        let sandbox = try!(try!(read_dir(download.path())).next().unwrap()).path();
        let valid = try!(Template::validate(&sandbox));
        if valid {
            try!(fs::create_dir_all(&template_dir));
            try!(rename(sandbox, template_dir));
        }
        Ok(valid)
    }

    pub fn list() -> Result<Vec<String>> {
        let mut names = vec![];
        let template_dir = try!(templates_dir());
        for entry in try!(fs::read_dir(template_dir)) {
            let e = try!(entry);
            if let Some(name) = e.file_name().to_str() {
                names.push(name.to_owned());
            }
        }
        Ok(names)
    }

    pub fn delete(tag: &str) -> Result<bool> {
        let template_dir = try!(templates_dir()).join(tag);
        if !template_dir.exists() {
            return Ok(false);
        }
        try!(fs::remove_dir_all(template_dir));
        Ok(true)
    }

    /// Resolve template
    pub fn get(path: &Path) -> Result<Template> {
        match find(&path, DEFAULTS) {
            Ok(Some(defaults)) => {
                Ok(Template {
                    defaults: defaults,
                    path: path.join(TEMPLATE_DIR),
                })
            }
            _ => Err(Error::DefaultsNotFound),
        }
    }

    /// resolve context
    fn context(&self) -> Result<BTreeMap<String, String>> {
        let map = try!(parse_defaults(self.defaults.as_path()));
        let resolved = try!(interact(&map));
        Ok(resolved)
    }

    /// Apply template
    pub fn apply(&self, target: &Path) -> Result<()> {
        let ctx = try!(self.context());
        let data = Context::wraps(&ctx);

        // apply handlebars processing
        let apply = |path: &Path, hbs: &mut Handlebars| -> Result<()> {

            // /tmp/download_dir/templates
            let scratchpath = &format!("{}{}", self.path.to_str().unwrap(), MAIN_SEPARATOR)[..];

            // path relatived based on scratch dir
            let localpath = path.to_str()
                .unwrap()
                .trim_left_matches(scratchpath);

            // eval path as template
            let evalpath = try!(hbs.template_render(&localpath, &ctx));

            // rewritten path, based on target dir and eval path
            let targetpath = target.join(evalpath);

            if path.is_dir() {
                try!(fs::create_dir_all(targetpath))
            } else {
                let mut file = try!(File::open(path));
                let mut s = String::new();
                try!(file.read_to_string(&mut s));
                if targetpath.exists() {
                    // open file for reading and writing
                    let mut file = try!(OpenOptions::new().append(false).write(true).read(true).open(&targetpath));

                    // get the current content
                    let mut current_content = String::new();
                    try!(file.read_to_string(&mut current_content));

                    // get the target content
                    let template_eval = try!(hbs.template_render(&s, &ctx));

                    // if there's a diff prompt for change
                    if template_eval != current_content {
                        let keep = try!(prompt_diff(current_content.as_ref(), template_eval.as_ref()));
                        if !keep {
                            // force truncation of current content
                            let mut file = try!(File::create(targetpath));
                            try!(file.write_all(template_eval.as_bytes()));
                        }
                    }
                } else {
                    let mut file = try!(File::create(targetpath));
                    try!(hbs.template_renderw(&s, &data, &mut file));
                }
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

fn prompt_diff(current: &str, new: &str) -> io::Result<bool> {
    let mut answer = String::new();
    println!("local changes exists in file <file>");
    difference::print_diff(current, new, "\n");
    print!("local changes exists. do you want to keep them?  [y]: ");
    try!(io::stdout().flush());
    try!(io::stdin().read_line(&mut answer));
    let trimmed = answer.trim();
    if trimmed.is_empty() || trimmed != "n" {
        Ok(true)
    } else {
        Ok(false)
    }

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

fn parse_defaults(p: &Path) -> Result<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();
    let mut f = try!(File::open(p));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    let values = s.lines()
        .filter(|l| !l.starts_with("#"))
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
/// to resolve the parameters that can not be inferred from env
fn interact(defaults: &BTreeMap<String, String>) -> Result<BTreeMap<String, String>> {
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
    use regex::Regex;
    use super::*;

    #[test]
    fn test_bars_upper() {
        let mut map = BTreeMap::new();
        map.insert("name".to_owned(), "porteurbars".to_owned());
        assert_eq!("Hello, PORTEURBARS",
                   bars().template_render("Hello, {{upper name}}", &map).unwrap());
    }

    #[test]
    fn test_bars_lower() {
        let mut map = BTreeMap::new();
        map.insert("name".to_owned(), "PORTEURBARS".to_owned());
        assert_eq!("Hello, porteurbars",
                   bars().template_render("Hello, {{lower name}}", &map).unwrap());
    }
}
