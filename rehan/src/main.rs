use rehan::*;
use std::io::Write;

fn main() -> Result<(), Error> {
    let mut args = std::env::args().skip(1);
    let file = args.next().ok_or(Error::MissingFile)?;
    let vars = parse::parse_args(args)?;
    let doc = parse::parse_doc(file)?.format(vars)?;

    let mut file = std::fs::File::create_new(&doc.file_name)?;
    file.write_all(doc.content.as_bytes())?;
    Ok(())
}
