#[derive(thiserror::Error, Debug)]
pub enum FilteError {
    #[error("No such command {0}")]
    UnkownCommand(String),
    #[error("Closed unopened command")]
    NeedlessClose,
    #[error("Missing command")]
    MissingCommand,
    #[error("Not closing command")]
    MissingClose,

    #[error(transparent)]
    RegexSyntax(#[from] regex::Error),
    #[error(transparent)]
    GlobSyntax(#[from] glob::PatternError),

    #[error("No such filter mode {0}'")]
    NoFilter(char),

    #[error("Missing text")]
    MissingText,

    #[error("Missing mode")]
    MissingMode,
}

#[derive(Debug)]
pub enum Mode {
    Is(String),
    Starts(String),
    Ends(String),
    Includes(String),
    Regex(regex::Regex),
    Glob(glob::Pattern),
}

#[derive(Debug)]
pub struct RawFilter {
    filter: Mode,
    invert: bool,
}

#[derive(Debug)]
pub enum Filter {
    Raw(RawFilter),
    And(Vec<Filter>),
    Or(Vec<Filter>),
}

impl Filter {
    pub fn compare(&self, text: &str) -> bool {
        match self {
            Filter::Raw(r) => r.compare(text),
            Filter::Or(rs) => rs.into_iter().any(|r| r.compare(text)),
            Filter::And(rs) => rs.into_iter().all(|r| r.compare(text)),
        }
    }
}

impl RawFilter {
    pub fn compare(&self, text: &str) -> bool {
        self.invert
            != match &self.filter {
                Mode::Is(rf) => rf == text,
                Mode::Starts(rf) => text.starts_with(rf),
                Mode::Ends(rf) => text.ends_with(rf),
                Mode::Includes(rf) => text.contains(rf),
                Mode::Regex(rf) => rf.is_match(text),
                Mode::Glob(rf) => rf.matches(text),
            }
    }
}

pub mod reader;
