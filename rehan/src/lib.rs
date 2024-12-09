pub use std::collections::HashMap;
use std::path::PathBuf;
use strfmt::strfmt;

type Vars = HashMap<String, String>;

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
}

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

fn upper_case_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}


impl Transform {
    fn transform(&self, value: String) -> Result<String, Error> {
        use Transform::*;
        match self {
            UpperCaseFirst => {
                Ok(upper_case_first(&value))
            }
            AllUpperCase => {
                Ok(value.to_uppercase())
            }
            AllLowerCase => {
                Ok(value.to_lowercase())
            }
            IsInt => {
                value.parse::<i128>()?;
                Ok(value)
            }
            IsNumber => {
                value.parse::<f64>()?;
                Ok(value)
            }
            IsGreaterThan(cmp) => {
                let num = value.parse::<f64>()?;
                if num > *cmp {
                    Ok(value)
                } else {
                    Err(Error::NotGreaterThan(num, *cmp))
                }
            }
            IsSmallerThan(cmp) => {
                let num = value.parse::<f64>()?;
                if num < *cmp {
                    Ok(value)
                } else {
                    Err(Error::NotSmallerThan(num, *cmp))
                }
            }
            IsNumberInRange(cmp_low, cmp_high) => {
                let num = value.parse::<f64>()?;
                if num <= *cmp_low && num >= *cmp_high {
                    Ok(value)
                } else {
                    Err(Error::OutOfRange(num, *cmp_low, *cmp_high))
                }

            }
        }
    }
}

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

impl Directive {
    fn act(self, doc: &mut RuntimeDoc) -> Result<(), Error> {
        use Directive::*;
        match self {
            Filename { expr } => {
                doc.file_name = PathBuf::from(strfmt(&expr, &doc.vars)?);
            }
            Input { name, transforms } => {
                let value = doc
                    .vars
                    .get(&name)
                    .or(doc.vars.get(&doc.input_ns.to_string()))
                    .ok_or(Error::MissingInput(doc.input_ns, name.clone()))?
                    .clone();
                let value = transforms
                    .iter()
                    .try_fold(value, |v, tra| tra.transform(v))?;
                doc.input_ns+=1;
                doc.vars.insert(name, value);
            }
            Format{name, expr} => {
                doc.vars.insert(name, strfmt(&expr, &doc.vars)?);
            }
            Set{name, from, transforms} => {
                let value = doc.vars.get(&from).ok_or(Error::MissingVar(from))?.clone();
                let value = transforms
                    .iter()
                    .try_fold(value, |v, tra| tra.transform(v))?;
                doc.vars.insert(name, value);
            }
        }
        Ok(())
    }
}

// parse .rehen. file into RawDoc
pub struct RawDocument {
    pub file_name: PathBuf,
    // directives defines for document
    pub directives: Vec<Directive>,
    // content before rehan processing
    pub actual_content: String,
}

// directives modify runtime doc content
pub struct RuntimeDoc {
    pub file_name: PathBuf,
    pub vars: Vars,
    input_ns: usize,
}

// document is built after all directives are executed
pub struct Document {
    pub file_name: PathBuf,
    pub content: String,
}

impl RawDocument {
    pub fn format(self, inputs: Vars) -> Result<Document, Error> {
        let directives = self.directives;
        let original_content = self.actual_content;
        let mut doc = RuntimeDoc {
            input_ns: 1,
            file_name: self.file_name,
            vars: inputs,
        };

        for directive in directives {
            directive.act(&mut doc)?;
        }
        Ok(Document {
            file_name: doc.file_name.clone(),
            content: strfmt(&original_content, &doc.vars)?,
        })
    }
}

