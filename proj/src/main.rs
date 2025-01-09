use proj::Error::{self, *};

fn exec() -> Result<(), Error> {
    let proj_file = std::env::var("PROJ_FILE").or(Err(MissingProjFile))?;
    let proj_cont = std::fs::read_to_string(proj_file)?;
    let config: proj::Config = toml::from_str(&proj_cont)?;

    let name = std::env::args().skip(1).next().ok_or(MissingProjectname)?;
    config.execute(name)?;
    Ok(())
}

fn main() {
    match exec() {
        Err(err)=> {
            eprintln!("{err}");
            std::process::exit(1);
        },
        Ok(()) => {}
    }
}
