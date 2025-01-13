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
    TomlDeserializeError(#[from] toml::de::Error),
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct CaseinsString(String);

impl PartialEq for CaseinsString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase().eq(&other.0.to_lowercase())
    }
}

impl std::hash::Hash for CaseinsString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state)
    }
}

impl Eq for CaseinsString { }

// TODO: make String newtype where impl Hash, Eq ignores case
#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config(
    //#[serde(deserialize_with = "lowercase_map")]
    pub HashMap<CaseinsString, Project>
);

impl Config {
    pub fn execute(mut self, name: String) -> Result<(), Error> {
        match self.0.remove(&CaseinsString(name.clone())) {
            None => Err(Error::UnknownProject(name)),
            Some(proj) => proj.execute(name)
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub enum Project {
    Posix(Posix),
    Nix(Nix),
}

impl Project {
    pub fn execute(self, name: String) -> Result<(), Error> {
        match self {
            Project::Nix(n) => n.execute(name),
            Project::Posix(n) => n.execute(name),
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Nix {
    pub shell: CString,
    pub packages: Vec<CString>,
    pub vars: HashMap<String, String>,
    pub dir: String,
    pub config_file: Option<CString>, //TODO
}

impl Nix {
    pub fn build_args(shell: CString, deps: Vec<CString>) -> Vec<CString> {
        [shell, c"--packages".to_owned()].into_iter().chain(deps).collect()
    }
    pub fn execute(self, name: String) -> Result<(), Error> {
        nix::unistd::chdir(&std::path::PathBuf::from(&self.dir))?;
        let vars: Vec<_> = std::env::vars()
            .into_iter()
            .chain(self.vars)
            .chain([("COMPUTER_NAME".to_owned(), name)])
            .map(|(k, v)| format!("{k}={v}"))
            .map(std::ffi::CString::new)
            .map(Result::unwrap)
            .collect();

        nix::unistd::execvpe(
            c"nix-shell",
            &Nix::build_args(self.shell, self.packages),
            &vars
        )?;
        Ok(())
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Posix {
    pub shell: CString,
    pub shell_args: Vec<CString>,
    pub vars: HashMap<String, String>,
    pub dir: String,
}

impl Posix {
    pub fn execute(self, name: String) -> Result<(), Error> {
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
