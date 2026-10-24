use crate::*;

#[derive(Debug)]
pub enum CdFilter {
    And,
    Or,
    Not,
    Close,
    Raw(String),
}

impl CdFilter {
    fn parse(arg: String) -> CdFilter {
        match arg.as_ref() {
            "and[" => CdFilter::And,
            "not[" => CdFilter::Not,
            "or[" => CdFilter::Or,
            "]" => CdFilter::Close,
            _ => CdFilter::Raw(arg),
        }
    }
}

pub fn parse(args: impl Iterator<Item = String>) -> Result<Filter, FilteError> {
    let mut cmds = args.map(CdFilter::parse);
    read_next(&mut cmds)
}

fn read_next(args: &mut impl Iterator<Item = CdFilter>) -> Result<Filter, FilteError> {
    let arg = args.next().ok_or(FilteError::MissingCommand)?;
    match arg {
        CdFilter::Not => read_not(args),
        CdFilter::And => read_many(args, Filter::And),
        CdFilter::Or => read_many(args, Filter::Or),
        CdFilter::Close => Err(FilteError::NeedlessClose),
        CdFilter::Raw(s) => Filter::try_from(s),
    }
}

fn read_not(args: &mut impl Iterator<Item = CdFilter>) -> Result<Filter, FilteError> {
    let out = read_next(args)?;
    match args.next() {
        None => Err(FilteError::MissingCommand),
        Some(CdFilter::Close) => Ok(Filter::Not(Box::new(out))),
        Some(_) => Err(FilteError::TooManyFilters),
    }
}

fn read_many(
    args: &mut impl Iterator<Item = CdFilter>,
    cnv: fn(Vec<Filter>) -> Filter,
) -> Result<Filter, FilteError> {
    let mut out = Vec::new();
    loop {
        let arg = args.next().ok_or(FilteError::MissingClose)?;
        out.push(match arg {
            CdFilter::Not => read_not(args),
            CdFilter::And => read_many(args, Filter::And),
            CdFilter::Or => read_many(args, Filter::Or),
            CdFilter::Close => break,
            CdFilter::Raw(r) => Filter::try_from(r),
        }?);
    }
    Ok(cnv(out))
}

impl TryFrom<String> for Filter {
    type Error = FilteError;
    fn try_from(tx: String) -> Result<Filter, FilteError> {
        use Mode::*;
        let mut chars = tx.chars();
        let mut mode = chars.next().ok_or(FilteError::MissingText)?;
        let mut invert = mode == 'i';
        if mode == 'i' {
            mode = chars.next().ok_or(FilteError::MissingMode)?;
        }
        let pattern: String = chars.collect();
        let filter = match mode {
            '=' => Is(pattern),
            's' | '^' => Starts(pattern),
            'z' | '$' => Ends(pattern),
            'h' | '+' => Includes(pattern),
            'e' | '-' => {
                invert = !invert;
                Includes(pattern)
            }
            'r' | '.' => Regex(pattern.try_into()?),
            'g' | '?' => Glob(glob::Pattern::new(&pattern)?),
            other => Err(FilteError::NoFilter(other))?,
        };
        Ok(Filter::Raw(RawFilter { filter, invert }))
    }
}
