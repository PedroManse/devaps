use filte2_rs::*;
use std::io::{self, Write, BufRead};

const HELP: &'static str = "
usage filter [i](mode)(pattern)

excludes lines that don't fit the patterns with speficied modes

if 'i' is the prefix of a mode the filter will exclude lines the *do* fit the pattern

modes:
=   : equals $pattern
^|s : starts with $pattern
$|z : ends with $pattern
+|h : includes $pattern
-|e : excludes $pattern (shorthand for i+)
.|r : matches regex $pattern
?|g : matches glob $pattern

filters can be grouped with and[ ... ], or[ ... ] or not[ . ] to execute multiple filters at once with logical joinings in them

the only way to print this text is to execute with no arguments
";

fn help() -> ! {
    eprintln!("{}", HELP);
    std::process::exit(0)
}

fn main() -> eyre::Result<()> {
    let mut stdout = io::stdout().lock();
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        help();
    }
    let filters = reader::parse(args.into_iter())?;

    for line in io::stdin().lock().lines() {
        let line = line?;
        if filters.compare(&line) {
            writeln!(stdout, "{}", line).inspect_err(die_on_pipe)?;
        }
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

