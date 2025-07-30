use todo_rs::*;
use conf::{Config, ConfigRaw};

use self::filstu::{DPath, FPath, Node};
use self::out::{JSONReport, TextReport};

fn main() -> Result<(), TDError> {
    let config_path = rev_find_config()
        .or(find_global_config())
        .ok_or(TDError::ConfigNotFound)?;
    let config_text = std::fs::read_to_string(config_path)?;
    let cfg: ConfigRaw = toml::from_str(&config_text)?;
    let cfg: Config = cfg.try_into()?;

    let flst = filstu::read_dir_filtered(".", &cfg)?;

    let args: Vec<_> = std::env::args().skip(1).collect();
    let xs: Vec<&str> = args.iter().map(|f|f.as_str()).collect();
    match &xs[..] {
        ["--tree"] | [] => {
            display_report(flst, cfg)
        }
        ["--json"] => {
            make_json_report(flst, cfg)
        }
        ["--text"] => {
            make_text_report(flst, cfg)
        }
        _=>{
            panic!()
        }
    }
}

fn display_report(flst: Node<FPath, DPath>, cfg: Config)  -> Result<(), TDError>{
    let tds = out::show_todos(flst, &cfg);
    println!("{tds}");
    Ok(())
}

fn make_json_report(flst: Node<FPath, DPath>, cfg: Config) -> Result<(), TDError> {
    let jsout: JSONReport = out::make_report(flst, &cfg)?;
    let out = serde_json::to_string(&jsout)?;
    print!("{out}");
    Ok(())
}

fn make_text_report(flst: Node<FPath, DPath>, cfg: Config) -> Result<(), TDError> {
    let out: TextReport = out::make_report(flst, &cfg)?;
    print!("{out}");
    Ok(())
}
