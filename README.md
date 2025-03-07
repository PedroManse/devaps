# Programs and stuff

# CTC
Converts characters to unicode indicies and vice-versa<br>

```shell
$ctc hello
h: 104
e: 101
l: 108
l: 108
o: 111

$ctc 104 101 108 108 111
104: h
101: e
108: l
108: l
111: o
```

With "-1" as the first argument, you can make the output better for piping

```shell
$ctc -1 104 101 108 108 111
h
e
l
l
o

$ctc -1 hello
104
101
108
108
111
```

# Filte
Fiter input text with regex, glob and simple patterns<br>
Include only the lines that match specified filters
`[i](mode)(pattern)`

## Modes (filter patterns)

mode        | symbol | char
------------|--------|-------
equal       | `=`    |
starts with | `^`    | `s`
ends with   | `$`    | `z`
includes    | `+`    | `h`
excludes    | `-`    | `e`
regex       | `.`    | `r`
glob        | `?`    | `g`

## Invert
You can prefix any mode with `i` to invert it's result (if the pattern matches, exclude the line)

## Groups
group[ filters... ]

### And
`and[ ... ]`
Only if all filters match, return a match<br>
group can short-circuit

### Or
`or[ ... ]`
If any filters match, return a match<br>
group can short-circuit

### Not
`not[ . ]`
Invert _the_ filter in group

<h2><a name="filte-examples">Examples</a></h2>
### Remove all directories
Remove all directories from [lsr](github.com/pedromanse/devaps#lsr) output
`lsr | filte i$/`

### List all files with a specific extensions
`lsr | filte or[ $.js $.ts ]`

# Fpwd
Fancy print working directory (daemon / standalone)<br>
Rename and colour directories with a simple config file

## config
config file found in `$FPWDRS_CONFIG` or `$HOME/.config/fpwd.lsp`. Socket path defined by `$FPWDRS_SOCKET_NAME`.

```lisp
(
    (
        ( from . "old-substring" ) ( to . "new-substring" )
    )
    (
        ( from . "new-substring" ) ( to . "newer-substring" )
        ( replace_n . 1 )
    )
)
```

This file would replace any occurrence of `old-substring` with `new-substring`
and all occourences of `new-substring` with `newer-substring`

Any substring `\\e` will be replaced with `U+001B` to facilitate ANSI escape sequences

Without a config file, the only change that would happed in `$HOME` to `~`

## fpwd-daemon
Tecnically not a daemon, since it doesn't fork

Opens a socket in `/tmp/fpwd-rs.sock`, parses the config file and listens indefinatly
Every line is parsed as a pwd output and printed after formatting is done

## pwd-rs
Reads the config file, the PWD variable and formats it accordingly

# Gs2
Report git repo status in a single line<br>
` {branch} {symbols}`

The branch is displayed underlined and in purple, and the symbols are always displayed in this order

symbol | meaning                                                  | color
-------|----------------------------------------------------------|--------
Ü      | clean (no other symbol to display)                       | green
+      | untracked (new) file                                     | red
→      | any tracked changes                                      | red
x      | merge conflict                                           | red
-      | untracked deleted files                                  | red
←      | untracked renamed file                                   | red
*      | untracked patches to files                               | red
↕      | local and remote HEAD diverge (may need to push or pull) | inverted red, blinking

# Lsr
Recursively list files and directories, one by line<br>
Lists everything from the directories sent as arguments, otherwise list current directory

Keeps all directories prefixes, even "./" when listing current dir

# Proj
Instantiates a shell with several characteristics defined by a configuration file<br>
With shell.nix, the only use for this is making global shortcuts for directories

compiled binary as `p`

## Posix config
name        | config
------------|-----------------------------------
shell       | shell to be instantiated
shell\_args | list of arguments to send to shell
vars        | map of env variables to define
dir         | directory to instantiate shell

## Nix config
Nix-specific features not developed yet, simply use the Posix config

## Config file
Loaded TOML file from `$PROJ_FILE`

```toml
[DevAps.Posix]
shell = "bash"
shell_args = []
dir = "/path/to/project/"
vars = {}

[srrs.Posix]
shell = "zsh"
shell_args = ["-i"]
dir = "/path/to/project/"
vars = {
    DATABASE_URL="..."
}
```

# Hottie
Re-execute command on file update (it's kinda shit rn, ngl)

# Lev
Get the levenshetein distance from multiple string to a target<br>
The results are not sorted

```shell
$lev 123 122 133 3141
1 122
1 133
3 3141
0 123
```

# Rehan
Rust-based easy handlebars<br>

## Syntax
Files are split in two, the directives section and the document. All directives
start with "#" and serve to acquire or modify information, and the last
directive must be `#done`. After the last directive the document content
starts, every `{}` pair will be replaed with the variable defined within and
every `{{` is swapped with `{` and `}}` with `}`

## Directives
The [Filename](#filename), [Set](#set) and [Format](#format) directives may
take experssions with `{}` that will be replaced by variables during formatting

### Filename
Define the filename of the formatted document to be output

> `{variables}` parsed

`#filename {title}-{author}.tex`

### Input
Define a variable to be input when formatting the file and how to modify or fitler it

`#input doc_id [IsInt()]`

### Format
Create a variable based on the expression provided

`#format global_regex /{regex}/g`

> `{variables}` parsed

### Set
Clone a variable and modify it with [transformers](#transformers)

`#set name_upper name_lower [UpperCaseFirst()]`

## Transformers
Modifiers are used in the #set directive to modify or filter strings. All
transformers need () after their names, even if they don't require any
arguments

transformers    | input    | action
----------------|----------|-----------------------
UpperCaseFirst  |          | uppercase first letter
AllUpperCase    |          | uppercase all letters
AllLowerCase    |          | lowercase all letters
IsInt           |          | check if is integer
IsNumber        |          | check if is number
IsSmallerThan   | f64      | check if number is smaller than X
IsGreaterThan   | f64      | check ir number is greater than X
IsNumberInRange | f64, f64 | check if number is in range \[X, Y\] (inclusive)

## Usage
```shell
$rehan <file> [<variable_name>:<variable_value>...] [<variable_value>...]
```

If variables are defined without names, they are sent in the order the input directives are defined

## Rehan-prepare

Since, to make a rehan file, you'd need to duplicate all `{}` pairs,
`rehan-prepare` does that, adds the `#done` directive at the begining of the
file and saves the document to `{file}.rehan.{ext}`.

# Rot
I don't remember<br>
Rot is a simpler `.dot` format parser that exports to `.dot` and other GraphViz supported formats

# Runner
Start the current project<br>
Based on `$HOME/.config/runner.cfg` understand the current project and execute it

The config file is split on scopes by double newlines. Each scope can have
several config operations. The configuration operations can define which files
need to exist for the scope to be valid  (`has=<file>`), which command to
execute (`exec=<command>`), which env variables to define (`env=<variable>`),
and to search for a file (`findAny=<file>`).

Each operation can be defined multiple times, and the `findAny` operation
defined the `$found` env variable, usable in `exec` operations. Each `exec`
operation will be executed in order of definition as a shell command

```conf
Rust
has=Cargo.toml
exec=Cargo run

Golang
findAny=main.go
findAny=project.go
findAny=run.go
exec=go run $found
```

# Size
Measure the size of a directories or files<br>
```shell
$size .
1314.45Mib
```

If no directories or files are provided, no default operations will occour.

If the size to be reported is over 10240 times the current unit, the next one is used.
Avaliabe units are: Bytes (`b`), Kibibytes (`Kib`), Mebibytes (`MiB`) and Gibibyte (`GiB`).

# Timer
## Create several timers with optional labels
You can split the timer from the label with a `-`, a `:` or a `)`

```shell
$timer 103-cake 8493.01:'torrent download' 193.20 41.13:'thing'
1: cake: completed!
2: torrent download: 7919.0709
3: timer: completed!
4: thing: completed!
```

# Tmpl
Searches for shell scripts in `$TMPLRS_DIR` or `$HOME/Templates/tmpl-rs` and executes it, passing on extra arguments

# Todo
Newest version doesn't exist yet.

# Sonar
Recursively find all links in a webpage

# Tester
Boring

