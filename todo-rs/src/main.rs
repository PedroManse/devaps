use std::fmt::Display;

use todo_rs::*;

use self::conf::{Config, ConfigRaw};
use self::filstu::Node;

fn main() -> Result<(), TDError> {
    let config_path = rev_find_config().or(find_global_config()).ok_or(TDError::ConfigNotFound)?;
    let config_text = std::fs::read_to_string(config_path)?;
    let cfg: ConfigRaw = toml::from_str(&config_text)?;
    let cfg: Config = cfg.try_into()?;

    let x = filstu::read_dir_fitered(".", &cfg)?;
    let z = x.map(|d, fs|d, |f|{
        let name = f.0.file_name().unwrap().to_str().unwrap();
        format!("{name} [0]")
    });
    println!("{z}");
    Ok(())
}
