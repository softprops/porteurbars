use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use errors::Result;

pub type Defaults = BTreeMap<String, String>;

/// parses key/value pairs from a target file
pub fn from_file<P>(path: P) -> Result<Defaults>
where
    P: AsRef<Path>,
{
    let mut s = String::new();
    try!(File::open(path)?.read_to_string(&mut s));
    Ok(from_string(s))
}

/// parses key/value pairs for a string of text
pub fn from_string(s: String) -> Defaults {
    s.lines()
        .filter(|l| !l.starts_with("#"))
        .map(|l| l.splitn(2, "=").collect::<Vec<_>>())
        .fold(
            BTreeMap::new(), |mut acc, pair| {
                if pair.len() == 2 {
                    if let Some(value) = pair[1].splitn(2, "#").next() {
                        acc.insert(pair[0].trim().to_owned(), value.trim().to_owned());
                    }
                }
                acc
            }
        )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;

    #[test]
    fn test_from_string() {
        let contents = String::from(
            "
FOO=bar # a comment

# another comment
BAZ = boom
",
        );
        let mut expected = BTreeMap::new();
        expected.insert(String::from("FOO"), String::from("bar"));
        expected.insert(String::from("BAZ"), String::from("boom"));
        assert_eq!(from_string(contents), expected)
    }
}
