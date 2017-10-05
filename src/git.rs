extern crate git2;
extern crate regex;
use errors::{ErrorKind, Result, ResultExt};

use git2::build::RepoBuilder;
use regex::Regex;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Url {
    Local(String),
    Github(String, String),
    Remote(String),
}

impl Url {
    /// supports two types git of repository urls
    /// local repositories that start with file://
    /// and github repositories ower/repo
    pub fn from_str(txt: &str) -> Result<Url> {
        lazy_static! {
            static ref LOCAL: Regex = Regex::new(r#"^file://(\S+)$"#).unwrap();
            static ref GH: Regex = Regex::new(r#"^([^\s/]+)/([^\s/]+?)$"#).unwrap();
            static ref REMOTE: Regex = Regex::new(
                r#"^(git[@|://].*)|(https://.*)|(http://.*)|(ssh://.*)$"#
            ).unwrap();
        }
        LOCAL
            .captures(txt)
            .map(|caps| Url::Local(caps.get(1).unwrap().as_str().to_owned()))
            .or_else(|| if REMOTE.is_match(txt) {
                Some(Url::Remote(txt.to_string()))
            } else {
                None
            })
            .or_else(|| {
                GH.captures(txt).map(|caps| {
                    Url::Github(
                        caps.get(1).unwrap().as_str().to_owned(),
                        caps.get(2).unwrap().as_str().to_owned(),
                    )
                })
            })
            .ok_or(ErrorKind::InvalidUri(txt.into()).into())
    }
}

/// clone a repository at a rev to a directory
/// a best attempt effort is made to authenticate
/// requests when required to support private
/// git repositories
pub fn clone<P, R>(repo: Url, dir: P, rev: R) -> Result<()>
where
    P: AsRef<Path>,
    R: Into<String>,
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
    fo.remote_callbacks(cb).download_tags(
        git2::AutotagOption::All,
    );
    let url = match repo {
        Url::Github(ref owner, ref repo) => {
            format!("git://github.com/{}/{}.git", owner, repo)
        }
        Url::Local(ref path) => path.to_owned(),
        Url::Remote(ref remote) => remote.to_owned(),
    };
    let revision = rev.into();
    RepoBuilder::new()
        .branch(&revision)
        .fetch_options(fo)
        .clone(&url, dir.as_ref())
        .chain_err(|| {
            format!("failed to clone repo {}@{}", &url, revision.clone())
        })?;

    debug!("cloned {:?} to {:?}", repo, dir.as_ref());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticated_ssh_url() {
        assert_eq!(
            Url::from_str("git@github.com:user/repo.git").unwrap(),
            Url::Remote(String::from("git@github.com:user/repo.git"))
        )
    }

    #[test]
    fn test_public_ssh_url() {
        assert_eq!(
            Url::from_str("git://github.com:user/repo.git").unwrap(),
            Url::Remote(String::from("git://github.com:user/repo.git"))
        )
    }

    #[test]
    fn test_https_url() {
        assert_eq!(
            Url::from_str("https://github.com/user/repo.git").unwrap(),
            Url::Remote(String::from("https://github.com/user/repo.git"))
        )
    }

    #[test]
    fn test_github_url() {
        assert_eq!(
            Url::from_str("user/repo").unwrap(),
            Url::Github(String::from("user"), String::from("repo"))
        )
    }

    #[test]
    fn test_local_uri() {
        assert_eq!(
            Url::from_str("file:///some/path/foo.git").unwrap(),
            Url::Local(String::from("/some/path/foo.git"))
        )
    }
}
