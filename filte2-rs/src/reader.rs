use crate::*;

#[derive(Debug)]
enum CdFilter {
    And,
    Or,
    Not,
    Close,
    Raw(String),
}

pub fn parse(args: impl Iterator<Item = String>) -> Result<Filter, FilteError> {
    let mut cmds = compile(args);
    next(&mut cmds)
}

fn compile(args: impl Iterator<Item = String>) -> impl Iterator<Item = CdFilter> {
    args.map(|arg| match arg.as_ref() {
        "and[" => CdFilter::And,
        "not[" => CdFilter::Not,
        "or[" => CdFilter::Or,
        "]" => CdFilter::Close,
        _ => CdFilter::Raw(arg),
    })
}

fn next(args: &mut impl Iterator<Item = CdFilter>) -> Result<Filter, FilteError> {
    let arg = args.next().ok_or(FilteError::MissingCommand)?;
    match arg {
        CdFilter::Not => csin(args, Filter::Not),
        CdFilter::And => cvec(args, Filter::And),
        CdFilter::Or => cvec(args, Filter::Or),
        CdFilter::Close => Err(FilteError::NeedlessClose),
        CdFilter::Raw(s) => raw(s),
    }
}

fn csin(
    args: &mut impl Iterator<Item = CdFilter>,
    cnv: fn(Box<Filter>) -> Filter,
) -> Result<Filter, FilteError> {
    let arg = args.next().ok_or(FilteError::MissingClose)?;
    let out = match arg {
        CdFilter::Not => csin(args, Filter::Not),
        CdFilter::And => cvec(args, Filter::And),
        CdFilter::Or => cvec(args, Filter::Or),
        CdFilter::Close => Err(FilteError::MissingCommand)?,
        CdFilter::Raw(r) => raw(r),
    }?;
    let closer = args.next().ok_or(FilteError::MissingClose)?;
    if let CdFilter::Close = closer {
        Ok(cnv(Box::new(out)))
    } else {
        Err(FilteError::TooManyFilters)
    }
}

fn cvec(
    args: &mut impl Iterator<Item = CdFilter>,
    cnv: fn(Vec<Filter>) -> Filter,
) -> Result<Filter, FilteError> {
    let mut out = Vec::new();
    loop {
        let arg = args.next().ok_or(FilteError::MissingClose)?;
        out.push(match arg {
            CdFilter::Not => csin(args, Filter::Not),
            CdFilter::And => cvec(args, Filter::And),
            CdFilter::Or => cvec(args, Filter::Or),
            CdFilter::Close => break,
            CdFilter::Raw(r) => raw(r),
        }?);
    }
    Ok(cnv(out))
}

fn raw(pattern: String) -> Result<Filter, FilteError> {
    RawFilter::try_from(pattern).map(Filter::Raw)
}

impl TryFrom<String> for RawFilter {
    type Error = FilteError;
    fn try_from(tx: String) -> Result<RawFilter, FilteError> {
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
        Ok(RawFilter { filter, invert })
    }
}
