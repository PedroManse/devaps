use crate::conf::Config;
use crate::filstu::{DPath, FPath, Node};
use crate::TDError;
use colored::Colorize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt::{self, Write};

#[derive(Debug, Serialize)]
pub struct Todo {
    pub pos: usize,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct FileTodos {
    pub from: PathBuf,
    pub todos: Vec<Todo>,
}

pub struct Report(Node<Result<FileTodos, TDError>, DPath>);
#[derive(Serialize)]
#[serde(untagged)]
enum D{
    File(FileTodos),
    Dir(JsonR),
}
#[derive(Serialize)]
struct JsonR(HashMap<PathBuf, D>);

#[derive(Serialize)]
pub struct JSONReport(Vec<JsonR>);
pub struct TextReport(Node<Result<FileTodos, TDError>, DPath>);

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

//impl fmt::Display for JSONReport {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        
//    }
//}

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
        let mut h = HashMap::new();
        value.0.map(|d, x|{
            let y = x.into_iter().filter_map(|f|f.atom()).map(|f|(&f).clone().unwrap());
            h.insert(d.0, () );
        }, |f|f);
        todo!()
        //JSONReport(match value.0.map(|d, _|d.0, |f|f.unwrap()) {
        //    Node::Atom(_)=>panic!(),
        //    Node::List(_, h)=>h,
        //})
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
