use std::path::{Path, PathBuf};

use regex::Regex;
use todo_rs::*;

use self::conf::{Config, ConfigRaw};

fn main() {
    //let config_file = rev_find_config().or(find_global_config());
    //println!("{config_file:?}");

    let cfg: Config = ConfigRaw{
        read_by_default: false,
        search_by_default: true,
        read_file_patterns: vec![".*\\.rs".to_string()],
        ignore_file_patterns: vec!["target/.+".to_string()],
        search_dir_patterns: vec![],
        ignore_dir_patterns: vec!["target/.+".to_string()],
        human_report: None,
        json_report: None,
    }.try_into().unwrap();
    println!("{cfg:?}");
    let x = filstu::read_dir_fitered(PathBuf::from(".").as_path(), &cfg);
    println!("{x:?}");
}
