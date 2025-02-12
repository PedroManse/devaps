use regex::Regex;
use serde::Deserialize;
use std::path::PathBuf;

use crate::filstu::PathFilter;

#[derive(Debug)]
pub struct FilePath(PathBuf);
#[derive(Debug)]
pub struct Patterns(Vec<Regex>);

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error(transparent)]
    InvalidRegex(#[from] regex::Error),
}

impl TryFrom<Vec<String>> for Patterns {
    type Error = ConfigError;
    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        value
            .into_iter()
            .map(|s| Regex::new(&s))
            .collect::<Result<Vec<_>, _>>()
            .map(Patterns)
            .map_err(Self::Error::from)
    }
}

impl From<String> for FilePath {
    fn from(value: String) -> Self {
        FilePath(PathBuf::from(value))
    }
}

impl Patterns {
    pub fn matches_any(&self, s: &str) -> bool {
        self.0.iter().any(|p| p.is_match(s))
    }
}

#[derive(Debug)]
pub struct Config {
    pub read_by_default: bool,
    pub search_by_default: bool,
    pub read_file_patterns: Patterns,
    pub ignore_file_patterns: Patterns,
    pub search_dir_patterns: Patterns,
    pub ignore_dir_patterns: Patterns,
    pub human_report: Option<FilePath>,
    pub json_report: Option<FilePath>,
}

impl PathFilter for Config {
    fn filter_dir(&self, path: &std::path::Path) -> bool {
        let exclude = path
                    .to_str()
                    .map(|path| self.ignore_dir_patterns.matches_any(path))
                    .unwrap_or(false);
        let include  = path
                .to_str()
                .map(|path| self.search_dir_patterns.matches_any(path))
                .unwrap_or(false);
        (self.search_by_default || include) && !exclude
    }
    fn filter_file(&self, path: &std::path::Path) -> bool {
        let exclude = path
                    .to_str()
                    .map(|path| self.ignore_file_patterns.matches_any(path))
                    .unwrap_or(false);
        let include  = path
                .to_str()
                .map(|path| self.read_file_patterns.matches_any(path))
                .unwrap_or(false);
        (self.read_by_default || include) && !exclude
    }
}

#[derive(Deserialize)]
pub struct ConfigRaw {
    pub read_by_default: bool,
    pub search_by_default: bool,
    pub read_file_patterns: Vec<String>,
    pub ignore_file_patterns: Vec<String>,
    pub search_dir_patterns: Vec<String>,
    pub ignore_dir_patterns: Vec<String>,
    pub human_report: Option<String>,
    pub json_report: Option<String>,
}

impl TryFrom<ConfigRaw> for Config {
    type Error = ConfigError;
    fn try_from(value: ConfigRaw) -> Result<Self, Self::Error> {
        Ok(Config {
            read_by_default: value.read_by_default,
            search_by_default: value.search_by_default,
            read_file_patterns: value.read_file_patterns.try_into()?,
            ignore_file_patterns: value.ignore_file_patterns.try_into()?,
            search_dir_patterns: value.search_dir_patterns.try_into()?,
            ignore_dir_patterns: value.ignore_dir_patterns.try_into()?,
            human_report: value.human_report.map(FilePath::from),
            json_report: value.json_report.map(FilePath::from),
        })
    }
}
