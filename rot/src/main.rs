#[allow(unused_imports)]
use rot::{builder, export, parse2, RotError};
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::path;

fn main() -> Result<(), RotError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use rot::graph;
    use super::*;
    fn parse_file(file: path::PathBuf) -> Result<Vec<parse2::Item>, RotError> {
        let code = fs::read_to_string(&file).unwrap();
        parse2::parse(code)
    }
    fn compile_file(items: Vec<parse2::Item>) -> Result<graph::Graph, RotError> {
        builder::build(items)
    }
    #[test]
    fn test_example_graphs() -> Result<(), RotError> {
        let tests = path::PathBuf::from("graphs");
        for test_file in fs::read_dir(tests).unwrap() {
            let file_name = test_file.unwrap().path();
            let parsed = parse_file(file_name)?;
            compile_file(parsed)?;
        }
        Ok(())
    }
}
