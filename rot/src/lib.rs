#[derive(thiserror::Error, Debug)]
pub enum RotError {
    #[error("No such node named {0}")]
    NoNodeName(String),
    #[error("No such node #{0}")]
    NoNodeId(usize),
    #[error("No such link #{0}")]
    NoLinkId(usize),
    #[error("Problem parsing .rot file\nunclosed item | buffer: {0}")]
    MissingInfo(String),
    #[error("Problem parsing .rot file\nilegal char for name | char: {0} buffer: {1}")]
    IlegalCharName(char, String),
    #[error("Problem parsing .rot file\nilegal char for item | char: {0}")]
    IlegalCharItem(char),
}

pub mod export;
pub mod graph;
pub mod parse;
