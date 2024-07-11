#[derive(thiserror::Error, Debug)]
pub enum RotError {
    // graph builder
    #[error("No such node named {0}")]
    NoNodeName(String),
    #[error("No such node #{0}")]
    NoNodeId(usize),
    #[error("No such link #{0}")]
    NoLinkId(usize),

    // parser
    #[error("Problem parsing .rot file\nUnclosed State {0:?}")]
    UnclosedState(parse2::Parser),
    #[error("Problem parsing .rot file\nUnclosed item | buffer: {0}")]
    MissingInfo(String),
    #[error("Problem parsing .rot file\nIlegal char for name | char: {0} buffer: {1}")]
    IlegalCharName(char, String),
    #[error("Problem parsing .rot file\nIlegal char for item | char: {0}")]
    IlegalCharItem(char),
    #[error("Problem parsing .rot file\nIlegal syntax on node link {0}")]
    LinkSyntaxError(char),
    #[error("Problem parsing .rot file\nProperty key without value {0}")]
    KeyWithoutValue(String),
    #[error("Problem in parsing .rot code\nValue: {0} missing a key")]
    ValueWithoutKey(String),
    #[error("Problem parsing .rot file\nKey {0} followed by ilegal char {0}, expecting either '\"' or whitespace")]
    DidntStartValue(String, char),
}

pub mod export;
pub mod graph;
pub mod parse2;
