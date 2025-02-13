use todo_rs::*;
use conf::{Config, ConfigRaw};

use self::out::{JSONReport, TextReport};

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

    let json_todos_report: JSONReport = out::make_report(flst.clone(), &cfg);
    //println!("{todos_report:?}");
    print!("{}",  serde_json::to_string(&json_todos_report).unwrap());
    Ok(())
}
