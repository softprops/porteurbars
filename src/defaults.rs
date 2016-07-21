use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::io::Read;

pub fn parse<P>(path: P) -> super::Result<BTreeMap<String, String>>
    where P: AsRef<Path>
{
    let mut map = BTreeMap::new();
    let mut f = try!(File::open(path));
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
