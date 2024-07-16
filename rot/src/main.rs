use rot::RotError;
use std::fs;

fn parse(code: String) -> Result<Vec<rot::parse2::Item>, RotError> {
    rot::parse2::parse(code)
}

fn graphs(items_pack: Vec<Vec<rot::parse2::Item>>) -> Result<rot::graph::Graph, RotError> {
    let mut graph = rot::graph::Graph::new();
    for items in items_pack {
        rot::builder::build(&mut graph, items)?;
    }
    Ok(graph)
}

fn help() -> ! {
    eprintln!("Usage:\n  rot [files.rot] [rot/svg/pdf]");
    std::process::exit(2)
}

//TODO figure out problem with prop if args: graphs/*
fn main() -> Result<(), RotError> {
    let mut args = std::env::args().skip(1);
    let export = args.next_back().unwrap_or_else(|| help());

    let items_pack: Vec<_> = args
        .map(|input| {
            let code = fs::read_to_string(input).unwrap();
            parse(code)
        })
        .collect::<Result<_, _>>()?;
    let graph = graphs(items_pack)?;

    use rot::export::to as exp;
    let out = match export.as_ref() {
        "rot" => exp::rot(&graph),
        "svg" => todo!(),
        "png" => todo!(),
        _ => help(),
    };

    println!("{}", out);
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
