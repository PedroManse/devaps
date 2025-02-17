use std::collections::HashMap;
use std::path::PathBuf;

pub mod conf;
pub mod filstu;
pub mod out;

const CONF_EXT: &'static str = "toml";
const CONF_NAME: &'static str = "config";
const LOCAL_CONF_NAME: &'static str = "todo";
pub const CONF_LEAF: &'static str = const_format::formatcp!("{CONF_NAME}.{CONF_EXT}");
pub const LOCAL_CONF_LEAF: &'static str = const_format::formatcp!(".{LOCAL_CONF_NAME}.{CONF_EXT}");

#[derive(thiserror::Error, Debug)]
pub enum TDError {
    #[error(transparent)]
    ConfigError(#[from] conf::ConfigError),
    #[error(transparent)]
    TomlDeError(#[from] toml::de::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Config file not found")]
    ConfigNotFound,
    #[error("Can't start JSON report from file, only from a directory")]
    TriedJSONReportFromFile,
    #[error(transparent)]
    JSONSerdeError(#[from] serde_json::Error),
}

pub fn rev_find_config() -> Option<PathBuf> {
    let mut cwd = std::env::current_dir().unwrap();
    loop {
        let Some(next) = cwd.parent() else { break None };
        let conf = cwd.clone().join(LOCAL_CONF_LEAF);
        if conf.exists() {
            break Some(conf);
        }
        cwd = next.to_path_buf();
    }
}

pub fn find_global_config() -> Option<PathBuf> {
    std::env::var("TODORS_CONFIG")
        .ok()
        .map(PathBuf::from)
        .or(directories::ProjectDirs::from("", "", "todo-rs")
            .map(|d| d.config_dir().to_path_buf().join(CONF_LEAF)))
}

impl crate::filstu::FPath {
    pub fn count_todos(&self, cfg: &crate::conf::Config) -> Result<usize, TDError> {
        let finders = cfg.get_todo_finders(self.0.as_path());
        let cont = std::fs::read_to_string(&self.0)?;
        Ok(finders
            .into_iter()
            .fold(0, |i, f| i + f.find_iter(&cont).count()))
    }

    pub fn report_todos(
        &self,
        cfg: &crate::conf::Config,
    ) -> Result<HashMap<usize, String>, TDError> {
        let finders = cfg.get_todo_finders(self.0.as_path());
        let cont = std::fs::read_to_string(&self.0)?;
        let mut matches = HashMap::new();
        for finder in finders {
            for mt in finder.find_iter(&cont) {
                let txt = mt.as_str().to_string();
                let start = mt.start();
                matches.insert(start, txt);
            }
        }

        Ok(matches)
    }
}
