use crate::conf::Config;
use crate::filstu::{DPath, FPath, Node};
use crate::TDError;
use colored::Colorize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt::{self, Write};

#[derive(Debug, Serialize, Hash, PartialEq, Eq, Clone)]
pub struct Todo {
    pub pos: usize,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct FileTodos {
    pub from: PathBuf,
    pub todos: Vec<Todo>,
}

pub struct Report(pub Node<Result<FileTodos, TDError>, DPath>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DMid {
    FileMid(Vec<Todo>, PathBuf),
    File(Vec<Todo>),
    Dir(JsonR),
}

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum D{
    File(Vec<Todo>),
    Dir(JsonR),
}
#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub struct JsonR(pub HashMap<PathBuf, D>);

#[derive(Serialize)]
pub struct JSONReport(JsonR);
pub struct TextReport(Node<Result<FileTodos, TDError>, DPath>);
type Nd = Node<Result<FileTodos, TDError>, DPath>;

pub mod json {
    use super::*;
    fn node_update_parent(node: Nd, parent: &mut HashMap<PathBuf, D>) -> Result<(), TDError> {
        match node {
            Node::Atom(f) => {
                let f = f?;
                parent.insert(f.from, D::File(f.todos));
            }
            Node::List(h, xs) => {
                let jr = filstu_to_jsonr(xs)?;
                parent.insert(h.0, D::Dir(jr));
            }
        };
        Ok(())
    }

    pub fn filstu_to_jsonr(xs: Vec<Nd>) -> Result<JsonR, TDError> {
        let mut r: HashMap<PathBuf, D> = HashMap::new();
        for x in xs.into_iter() {
            node_update_parent(x, &mut r)?;
        }
        Ok(JsonR(r))
    }
}

pub fn show_todos(flst: Node<FPath, DPath>, cfg: &Config) -> Node<String, DPath> {
    flst.map(
        |d, _| d,
        |f| {
            let count = f.count_todos(&cfg);
            let count = match count {
                Ok(0) => "0".green(),
                Ok(n) => n.to_string().yellow(),
                Err(x) => x.to_string().red(),
            };
            let name = match f.0.file_name().map(|f| f.to_string_lossy()) {
                Some(x) => x.into_owned(),
                None => "File not UTF-8".to_string(),
            };
            format!("{name} [{count}]")
        },
    )
}

// TODO: I'm calling .map/.map_ref (push action, not pull) on the same Node 3 times
impl fmt::Display for TextReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.0.map_ref(
            |_, _| (),
            |fl| {
                let Ok(fl) = fl else { panic!() };
                if fl.todos.is_empty() {
                    return Ok(());
                }

                // skip first directory
                let name = fl
                    .from
                    .into_iter()
                    .skip(1)
                    .collect::<PathBuf>()
                    .display()
                    .to_string();
                f.write_str(&name)?;
                f.write_char('\n')?;
                for t in &fl.todos {
                    f.write_char('\t')?;
                    f.write_str(&t.text)?;
                    //f.push('\n');
                }
                f.write_char('\n')
            },
        );
        for a in x.into_atoms() {
            a?;
        }
        Ok(())
    }
}

impl From<Report> for TextReport {
    fn from(value: Report) -> Self {
        TextReport(value.0)
    }
}

impl From<Report> for JSONReport {
    fn from(value: Report) -> Self {
        todo!()
    }
}

pub fn make_report<R: From<Report>>(flst: Node<FPath, DPath>, cfg: &Config) -> R {
    Report(flst.map(
        |d, _| d,
        |f| {
            let reports: Vec<Todo> = f
                .report_todos(&cfg)?
                .into_iter()
                .map(|(pos, text)| Todo { pos, text })
                .collect();
            let todo_entry = FileTodos {
                from: f.0,
                todos: reports,
            };
            Ok(todo_entry)
        },
    )).into()
}
