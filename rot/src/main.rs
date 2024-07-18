use rot::RotError;
use std::fs;
fn help() -> ! {
    eprintln!("Usage:\n  rot [files.rot] [rot/svg/pdf]");
    std::process::exit(2)
}

fn main() -> Result<(), RotError> {
    let mut args = std::env::args().skip(1);
    let export = args.next_back().unwrap_or_else(|| help());

    let mut graph = rot::graph::Graph::new();
    for input in args {
        let code = fs::read_to_string(input).unwrap();
        let items = rot::parse2::parse(code)?;
        rot::builder::build(&mut graph, items)?;
    }

    use rot::export::to as exp;
    match export.as_ref() {
        "rot" => exp::rot(&graph),
        "dot" => exp::dot(&graph),
        //"svg" => todo!(),
        //"png" => todo!(),
        _ => help(),
    }?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::{read_dir, read_to_string};
    use std::path::PathBuf;
    #[test]
    fn test_example_graphs() -> Result<(), RotError> {
        let tests = PathBuf::from("graphs");
        for test_file in read_dir(tests).unwrap() {
            let file_name = test_file.unwrap().path();
            let code = read_to_string(file_name).unwrap();
            build(parse(code)?)?;
        }
        Ok(())
    }
}
