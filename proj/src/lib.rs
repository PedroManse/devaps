pub use std::collections::HashMap;
use std::ffi::CString;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Can't find PROJ_FILE env var")]
    MissingProjFile,
    #[error("No such project {0:?}")]
    UnknownProject(String),
    #[error("Missing project name")]
    MissingProjectname,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    NixErr(#[from] nix::errno::Errno),
    #[error(transparent)]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error(transparent)]
    TomlDeserializeError(#[from] toml::de::Error),
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Config(pub HashMap<String, Project>);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Project {
    pub shell: CString,
    pub shell_args: Vec<CString>,
    pub vars: HashMap<String, String>,
    pub dir: String,
}

impl Config {
    pub fn execute(mut self, name: String) -> Result<(), Error> {
        match self.0.remove(&name) {
            None => Err(Error::UnknownProject(name)),
            Some(proj) => proj.execute(name)
        }
    }
}

impl Project {
    pub fn execute(&self, name: String) -> Result<(), Error> {
        nix::unistd::chdir(&std::path::PathBuf::from(&self.dir))?;
        let vars: Vec<_> = std::env::vars()
            .into_iter()
            .chain(self.vars.clone())
            .chain([("COMPUTER_NAME".to_owned(), name)])
            .map(|(k, v)| format!("{k}={v}"))
            .map(std::ffi::CString::new)
            .map(Result::unwrap)
            .collect();

        nix::unistd::execvpe(&self.shell, &self.shell_args, &vars)?;
        Ok(())
    }
}
