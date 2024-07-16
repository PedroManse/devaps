use rot::{
    parse2,
    builder,
    RotError
};
use std::fs;

fn main() -> Result<(), RotError> {
    let code = fs::read_to_string("graphs/n-1.rot").unwrap();
    let p = parse2::parse(code)?;
    let b = builder::build(p)?;
    println!("{b:#?}");
    Ok(())
}
