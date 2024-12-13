use rehan::*;
use std::io::Write;
use std::{fs, path};

fn make_rehan_filename(origin: &path::PathBuf) -> path::PathBuf {
    let mut ext = std::ffi::OsString::from("rehan.");
    ext.push(origin.extension().unwrap_or(std::ffi::OsStr::new("")));
    origin.with_extension(ext)
}

fn main() -> Result<(), Error> {
    std::env::args()
        .skip(1)
        .map(path::PathBuf::from)
        .map(|fl| (make_rehan_filename(&fl), fs::read_to_string(fl)))
        .try_for_each(|(dest, cont)| {
            let mut fl = fs::File::create_new(&dest)?;
            fl.write_all(b"#done\n")?;
            fl.write_all(cont?.replace("{", "{{").replace("}", "}}").as_bytes())
        })
        .map_err(Error::from)
}
