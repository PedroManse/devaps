use cal_rs::back;

fn main() -> Result<(), back::CalError> {
    let x = back::Date::now()?;
    println!("{x}");
    Ok(())
}
