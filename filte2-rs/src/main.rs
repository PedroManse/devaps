use filte2_rs::*;
use std::io::{stdin, BufRead};

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

filters can be grouped with and[ ... ] & or[ ... ] to execute multiple filters at once

the only way to print this text is to execute with no arguments
";

fn help() -> ! {
    eprintln!("{}", HELP);
    std::process::exit(0)
}

fn main() -> eyre::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        help();
    }
    let filters = reader::parse(args.into_iter())?;

    for line in stdin().lock().lines() {
        let line = line?;
        if filters.compare(&line) {
            println!("{line}");
        }
    }
    Ok(())
}
