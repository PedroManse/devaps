use std::path::PathBuf;

pub mod conf;
pub mod filstu;

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
    #[error("")]
    ConfigNotFound,
}

pub fn rev_find_config() -> Option<PathBuf> {
    let mut cwd = std::env::current_dir().unwrap();
    loop {
        let Some(next) = cwd.parent() else {
            break None
        };
        let conf = cwd.clone().join(LOCAL_CONF_LEAF);
        if conf.exists() {
            break Some(conf)
        }
        cwd = next.to_path_buf();
    }
}

pub fn find_global_config() -> Option<PathBuf> {
    std::env::var("TODORS_CONFIG").ok().map(PathBuf::from).or(
        directories::ProjectDirs::from("", "", "todo-rs").map(|d|d.config_dir().to_path_buf().join(CONF_LEAF))
    )
}


