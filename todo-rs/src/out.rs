use crate::conf::Config;
use crate::filstu::{DPath, FPath, Node};
use crate::TDError;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt::{self, Write};
use colored::Colorize;

#[derive(Debug, Serialize, Hash, PartialEq, Eq, Clone)]
struct Todo {
    pos: usize,
    text: String,
}

#[derive(Debug, Serialize)]
struct FileTodos {
    from: PathBuf,
    todos: Vec<Todo>,
}

pub struct Report(Node<Result<FileTodos, TDError>, DPath>);

#[derive(Serialize)]
pub struct JSONReport(json::JsonR);
pub struct TextReport(Node<Result<FileTodos, TDError>, DPath>);

mod json {
    use super::*;
    type Nd = Node<Result<FileTodos, TDError>, DPath>;
    #[derive(Debug, Serialize, PartialEq, Eq, Clone)]
    #[serde(untagged)]
    enum D{
        File(Vec<Todo>),
        Dir(JsonR),
    }
    #[derive(Debug, Serialize, PartialEq, Eq, Clone)]
    pub(super) struct JsonR(HashMap<PathBuf, D>);

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

    pub(super) fn filstu_to_jsonr(xs: Vec<Nd>) -> Result<JsonR, TDError> {
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

impl IntoReport<TextReport> for Report {
    type Error = TDError;
    fn into_report(self: Report, _: &Config) -> Result<TextReport, Self::Error> {
        Ok(TextReport(self.0))
    }
}

impl IntoReport<JSONReport> for Report {
    type Error = TDError;
    fn into_report(self: Report, _: &Config) -> Result<JSONReport, Self::Error> {
        match self.0 {
            Node::List(_, xs) => {
                json::filstu_to_jsonr(xs).map(JSONReport)
            },
            _=>{
                Err(TDError::TriedJSONReportFromFile)
            }
        }
    }
}

impl<RF> IntoReport<RF> for Report
where RF: From<Report>
{
    type Error = ();
    fn into_report(self, cfg: &Config) -> Result<RF, Self::Error> {
        Ok(self.into())
    }
}

pub trait IntoReport<R> {
    type Error;
    fn into_report(self, cfg: &Config) -> Result<R, Self::Error>;
}

pub fn make_report<R>(flst: Node<FPath, DPath>, cfg: &Config) -> Result<R, <Report as IntoReport<R>>::Error>
where Report: IntoReport<R>
{
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
    )).into_report(cfg)
}

