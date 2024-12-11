use crate::*;

impl Transform {
    pub fn transform(&self, value: String) -> Result<String, Error> {
        use Transform::*;
        match self {
            UpperCaseFirst => Ok(upper_case_first(&value)),
            AllUpperCase => Ok(value.to_uppercase()),
            AllLowerCase => Ok(value.to_lowercase()),
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

impl Directive {
    pub fn act(self, doc: &mut RuntimeDoc) -> Result<(), Error> {
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
                doc.input_ns += 1;
                doc.vars.insert(name, value);
            }
            Format { name, expr } => {
                doc.vars.insert(name, strfmt(&expr, &doc.vars)?);
            }
            Set {
                name,
                from,
                transforms,
            } => {
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
