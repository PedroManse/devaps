pub use std::collections::HashMap;
pub use strfmt::strfmt;
pub type Vars = HashMap<String, String>;
pub mod build;
pub mod parse;
mod reader;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    FmtError(#[from] strfmt::FmtError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Missing input {1} index {0}")]
    MissingInput(usize, String),
    #[error("Missing variable {0}")]
    MissingVar(String),
    #[error(transparent)]
    IsNotInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    IsNotNum(#[from] std::num::ParseFloatError),
    #[error("Value {0} is not greater than {1}")]
    NotGreaterThan(f64, f64),
    #[error("Value {0} is not smaller than {1}")]
    NotSmallerThan(f64, f64),
    #[error("Value {0} is out range [{1}, {2}]")]
    OutOfRange(f64, f64, f64),
    #[error("Unkown Directive {0}")]
    UnkownDirective(String),
    #[error("Directive {0} needs more arguments")]
    DirectiveMissingArgs(String),
    #[error("Transformer {0} was wrong argument count")]
    TransformWrongArgsCount(String),
    #[error("Unkown Transformer {0}")]
    UnkownTransform(String),
    #[error("Missing file argument")]
    MissingFile,
}

#[derive(Debug)]
pub enum Transform {
    UpperCaseFirst,
    AllUpperCase,
    AllLowerCase,
    IsInt,
    IsNumber,
    IsSmallerThan(f64),
    IsGreaterThan(f64),
    IsNumberInRange(f64, f64),
}

#[derive(Debug)]
pub enum Directive {
    Filename {
        expr: String,
    },
    Input {
        name: String,
        transforms: Vec<Transform>,
    },
    Format {
        name: String,
        expr: String,
    },
    Set {
        name: String,
        from: String,
        transforms: Vec<Transform>,
    },
}

// parse .rehen. file into RawDoc
#[derive(Debug)]
pub struct RawDocument {
    pub file_name: String,
    // directives defines for document
    pub directives: Vec<Directive>,
    // content before rehan processing
    pub actual_content: String,
}

// directives modify runtime doc content
#[derive(Debug)]
pub struct RuntimeDoc {
    pub file_name: String,
    pub vars: Vars,
    input_ns: usize,
}

// document is built after all directives are executed
#[derive(Debug)]
pub struct Document {
    pub file_name: String,
    pub content: String,
}

fn upper_case_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
