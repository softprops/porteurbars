use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::io::Read;

pub type Value = (String, Option<String>);

pub fn parse<P>(path: P) -> super::Result<BTreeMap<String, Value>>
    where P: AsRef<Path>
{
    let mut map = BTreeMap::new();
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    let values = s.lines()
        .filter(|l| !l.starts_with("#"))
        .map(|l| l.split("=").take(2).collect::<Vec<_>>())
        .collect::<Vec<Vec<&str>>>();
    for pair in values.iter() {
        if pair.len() == 2 {
            let values = pair[1].split("#").take(2).collect::<Vec<_>>();
            map.insert(pair[0].trim().to_owned(),
                       (values[0].trim().to_owned(),
                        values.into_iter().nth(1).map(|v| v.trim().to_owned())));
        }
    }

    Ok(map)
}
