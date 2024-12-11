use crate::*;

fn parse_transforms(line: &str) -> Result<Vec<Transform>, Error> {
    use Error::*;
    use Transform::*;
    line.split(&[')', '[', ']'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once("(").unwrap_or((s, "")))
        .map(|(t, args)| {
            let split_args: Vec<_> = args
                .split(",")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .collect();
            (t, split_args)
        })
        .map(|(transform, args)| {
            Ok(match (transform, &args[..]) {
                ("UpperCaseFirst", []) => UpperCaseFirst,
                ("AllUpperCase", []) => AllUpperCase,
                ("AllLowerCase", []) => AllLowerCase,
                ("IsInt", []) => IsInt,
                ("IsNumber", []) => IsNumber,
                ("IsSmallerThan", [max]) => {
                    let max: f64 = max.parse()?;
                    IsSmallerThan(max)
                }
                ("IsGreaterThan", [min]) => {
                    let min: f64 = min.parse()?;
                    IsGreaterThan(min)
                }
                ("IsNumberInRange", [min, max]) => {
                    let min: f64 = min.parse()?;
                    let max: f64 = max.parse()?;
                    IsNumberInRange(min, max)
                }
                ("IsNumberInRange" | "IsSmallerThan" | "IsGreaterThan", _) => {
                    return Err(TransformWrongArgsCount(transform.to_string()))
                }
                (_, _) => return Err(UnkownTransform(transform.to_string())),
            })
        })
        .collect()
}

fn parse_directive(line: &str) -> Result<Option<Directive>, Error> {
    use Directive::*;
    use Error::*;
    let mut line = line.splitn(2, " ");
    let directive_name = match line.next() {
        None => return Ok(None),
        Some(x) => x,
    };

    let directive_args = line.next();
    Ok(Some(match (directive_name, directive_args) {
        ("#comment", _) => return Ok(None),
        ("#filename", Some(expr)) => Filename {
            expr: expr.to_string(),
        },
        ("#input", Some(expr)) => {
            let (name, transforms) = expr
                .split_once(" ")
                .map(|(n, t)| (n, parse_transforms(t)))
                .ok_or(DirectiveMissingArgs(directive_name.to_string()))
                // in case of no transforms in directive
                .unwrap_or((expr, Ok(vec![])));
            Input {
                name: name.to_string(),
                transforms: transforms?,
            }
        }
        ("#format", Some(expr)) => {
            let (name, expr) = expr
                .split_once(" ")
                .ok_or(DirectiveMissingArgs(directive_name.to_string()))?;
            Format {
                name: name.to_string(),
                expr: expr.to_string(),
            }
        }
        ("#set", Some(expr)) => {
            let (name, from, transforms) = match expr.splitn(3, " ").collect::<Vec<&str>>()[..] {
                [a, b, c] => (a.to_string(), b.to_string(), c),
                _ => return Err(DirectiveMissingArgs(directive_name.to_string())),
            };
            Set {
                name,
                from,
                transforms: parse_transforms(transforms)?,
            }
        }
        ("#filename" | "#input" | "#format" | "#set", None) => {
            return Err(DirectiveMissingArgs(directive_name.to_string()))
        }
        (x, _) => return Err(UnkownDirective(x.to_string())),
    }))
}

pub fn parse(file_name: PathBuf) -> Result<RawDocument, Error> {
    let mut reader = reader::BufReader::open(&file_name)?;
    let mut buffer = String::new();
    let mut content = String::new();
    let mut directives = Vec::<Directive>::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        let line = line?.trim();
        if line == "#done" {
            break;
        }
        if let Some(directive) = parse_directive(line)? {
            directives.push(directive);
        }
    }

    while let Some(line) = reader.read_line(&mut buffer) {
        content += line?;
    }

    Ok(RawDocument {
        file_name,
        directives,
        actual_content: content,
    })
}
