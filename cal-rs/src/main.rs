use cal_rs::back;

fn main() -> eyre::Result<()> {
    let x = back::Date::now()?;
    println!("{x}");
    Ok(())
}

