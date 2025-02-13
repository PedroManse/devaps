use std::fmt::{self, format, Display};
use std::path::PathBuf;

use self::conf::{Config, ConfigRaw};
use self::filstu::{DPath, FPath, Node};
use colored::Colorize;
use todo_rs::*;

fn show_todos(flst: Node<FPath, DPath>, cfg: &Config) -> Node<String, DPath> {
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

#[derive(Debug)]
struct Todo {
    pos: usize,
    text: String,
}

#[derive(Debug)]
struct FileTodos {
    from: PathBuf,
    todos: Vec<Todo>,
}

type Report = Node<Result<FileTodos, TDError>, DPath>;

fn write_report(report: Report) -> String {
    let mut c = String::new();
    report.map(
        |_, _| (),
        |f| {
            let Ok(f) = f else { panic!() };
            if f.todos.is_empty() {
                return ();
            }

            // skip first directory
            let name = f
                .from
                .into_iter()
                .skip(1)
                .collect::<PathBuf>()
                .display()
                .to_string();
            c.push_str(&name);
            c.push('\n');
            for t in f.todos {
                c.push('\t');
                c.push_str(&t.text);
                //c.push('\n');
            }
            c.push('\n');
            ()
        },
    );
    c
}

fn list_todos(flst: Node<FPath, DPath>, cfg: &Config) -> Report {
    flst.map(
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
    )
}

fn main() -> Result<(), TDError> {
    let config_path = rev_find_config()
        .or(find_global_config())
        .ok_or(TDError::ConfigNotFound)?;
    let config_text = std::fs::read_to_string(config_path)?;
    let cfg: ConfigRaw = toml::from_str(&config_text)?;
    let cfg: Config = cfg.try_into()?;

    let flst = filstu::read_dir_fitered(".", &cfg)?;
    //println!("{flst}");
    //let todos_display = show_todos(flst, &cfg);
    //println!("{todos_display}");

    let todos_report = list_todos(flst, &cfg);
    //println!("{todos_report:?}");
    let x = write_report(todos_report);
    print!("{x}");
    Ok(())
}
