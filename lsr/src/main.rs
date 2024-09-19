use std::io::{self, Write};
type Result<T> = std::result::Result<T, io::Error>;

fn main() -> Result<()> {
    let mut stdout = io::stdout().lock();
    let mut dirs = 0;
    let args = std::env::args();
    args.skip(1).into_iter().try_fold((), |_, arg|{
        dirs+=1;
        let st = lsr::read_dir(arg.as_ref())?;
        write!(stdout, "{}", st).inspect_err(die_on_pipe)
    })?;
    if dirs==0 {
        let st = lsr::read_dir(".".as_ref())?;
        write!(stdout, "{}", st).inspect_err(die_on_pipe)?;
    }
    Ok(())
}

// in case piped to head, don't eprint on closed pipe
fn die_on_pipe(r: &io::Error)  {
    match r.kind() {
        io::ErrorKind::BrokenPipe=>std::process::exit(0),
        _=>(),
    };
}

