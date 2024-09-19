use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum Cons<A, H> {
    Atom(A),
    List(H, Vec<Cons<A, H>>),
}
pub type Entry = Cons<PathBuf, PathBuf>;

pub fn read_dir(dir: &Path) -> Result<Entry, std::io::Error> {
    if !dir.is_dir() {
        return Ok(Entry::Atom(dir.into()));
    }
    let out: Result<Vec<_>, _> = fs::read_dir(dir)?
        .map(|f| read_dir(&f?.path()))
        .collect();
    Ok(Entry::List(dir.to_owned(), out?))
}

use std::fmt;
impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display_entry(f, self, "")
    }
}

fn display_entry(
    f: &mut fmt::Formatter<'_>,
    e: &Entry,
    parent: &str,
) -> Result<(), fmt::Error> {
    f.write_str(parent)?;
    match e {
        Entry::Atom(name) => {
            let name = name.file_name().unwrap().to_str().unwrap();
            f.write_str(name)?;
            f.write_str("\n")?;
        }
        Entry::List(name, fs) => {
            let name = name
                .file_name()
                .and_then(|f|f.to_str())
                .unwrap_or(".");
            f.write_str(name)?;
            let down = format!("{parent}{name}/");
            f.write_str("/\n")?;
            for fl in fs.iter() {
                display_entry(f, fl, &down)?;
            }
        }
    };
    Ok(())
}


