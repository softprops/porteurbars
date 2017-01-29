extern crate git2;
extern crate regex;

use git2::build::RepoBuilder;
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
pub enum Url {
    Local(String),
    Github(String, String),
}

impl Url {
    /// supports two types git of repository urls
    /// local repositories that start with file://
    /// and github repositories ower/repo
    pub fn from_str(txt: &str) -> Option<Url> {
        lazy_static! {
            static ref LOCAL: Regex = Regex::new(r#"^file://(\S+)$""#).unwrap();
            static ref GH: Regex = Regex::new(r#"^([^\s/]+)/([^\s/]+?)$"#).unwrap();
        }
        GH.captures(txt)
            .map(|caps| {
                Url::Github(caps.get(1).unwrap().as_str().to_owned(),
                            caps.get(2).unwrap().as_str().to_owned())
            })
            .or_else(|| {
                LOCAL.captures(txt)
                    .map(|caps| Url::Local(caps.get(1).unwrap().as_str().to_owned()))
            })
    }
}

/// clone a repository at a rev to a directory
/// a best attempt effort is made to authenticate
/// requests when required to support private
/// git repositories
pub fn clone<P>(repo: Url, dir: P, rev: &str)
    where P: AsRef<Path>
{
    let mut cb = git2::RemoteCallbacks::new();
    let mut tried_sshkey = false;
    cb.credentials(move |url, username, cred_type| {
        if cred_type.contains(git2::USER_PASS_PLAINTEXT) {
            let cfg = git2::Config::open_default().unwrap();
            return git2::Cred::credential_helper(&cfg, url, username);
        }
        if cred_type.contains(git2::SSH_KEY) && !tried_sshkey {
            // If ssh-agent authentication fails, libgit2 will keep
            // calling this callback asking for other authentication
            // methods to try. Make sure we only try ssh-agent once,
            // to avoid looping forever.
            tried_sshkey = true;
            let username = username.unwrap();
            return git2::Cred::ssh_key_from_agent(&username);
        }
        Err(git2::Error::from_str("no authentication available"))
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb).download_tags(git2::AutotagOption::All);
    let url = match repo {
        Url::Github(ref owner, ref repo) => format!("git://github.com/{}/{}.git", owner, repo),
        Url::Local(ref path) => path.to_owned(),
    };
    RepoBuilder::new()
        .branch(rev)
        .fetch_options(fo)
        .clone(&url, dir.as_ref())
        .expect("cloned");

    debug!("cloned {:?} to {:?}", repo, dir.as_ref());
}
