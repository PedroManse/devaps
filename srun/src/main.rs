use std::path;
use std::collections;

// Commands
// . 'check' means check file
// = 'set' means set var
// $ 'exec' execute command

// Modifiers
// ! 'must' means exec/check *must* pass
// ? 'maybe' means boolean check on exec/check
// + 'if' means only execute if previous maybe passed
// - 'else' means only execute if previous maybe *didn't* pass

/*
[golang]
!. go.mod # <- must have a go.mod file

?. main.go # <- check if there's a main.go file, go run that
+ $ go run main.go # <- if so, go run that
- = pk $ cat go.mod | head -n1 | cut -d' ' -f 2 # else, get bin package name
- ?$ go build -o $pk # go build with package name
- + $ ./$pk # execute
*/

enum Command {
    Check(std::path::PathBuf),
    Exec(String),
    Set{name: String, cmd: String},
}

enum Modf {
    Nothing,
    Maybe{pass: Vec<Line>, fail: Vec<Line>},
    Must,
}

struct Line {
    cmd: Command,
    modf: Modf,
}

struct ExecScript {
    lines: Vec<Line>,
    env: collections::HashMap<String, String>,
}

struct Script {
    lines: Vec<Line>,
    checks: Vec<Line>,
}

fn main() {
}

