use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Edit {
    from: String,
    to: String,
    replace_n: Option<usize>,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct BDir ( String );

impl BDir {
    pub fn into_string(self) -> String {
        self.0.replace("\\e", "\x1b")
    }
    pub fn new(path: String) -> BDir {
        BDir( path )
    }
    pub fn edit(self, cfg: &Edit) -> BDir {
        BDir(
            self.0.replacen(&cfg.from, &cfg.to, cfg.replace_n.unwrap_or(999))
        )
    }
}

pub fn get_config() -> Result<Vec<Edit>, &'static str> {
    let home = std::env::var("HOME").or(Err("Can't read $HOME"))?;
    let filename = std::env::var("FPWDRS_CONFIG").or(Ok(home.clone()+"/.config/fpwd.lsp"))?;
    let file = std::fs::read_to_string(filename);
    match file {
        Ok(content) => {
            match serde_lexpr::from_str::<Vec<Edit>>(&content) {
                Err(x)=>{
                    eprintln!("{x:?}");
                    Err("Parse error in ~/.config/fpwd.lsp")
                },
                Ok(c)=>{
                    Ok(c)
                }
            }
        }
        Err(_) => {
            let tilde = "~".to_owned();
            Ok(vec![Edit {
                from: home,
                to: tilde,
                replace_n: None,
            }])
        }
    }
}

pub fn fancy_unwrap<T>(e: Result<T, &'static str>) -> T {
    match e {
        Ok(c) => c,
        Err(reason) => {
            eprintln!("pwd-rs ERROR: {reason}");
            std::process::exit(1)
        }
    }
}

