type Result = std::result::Result<(), std::io::Error>;

fn main() -> Result {
    let mut dirs = 0;
    let args = std::env::args();
    args.skip(1).into_iter().try_fold((), |_, arg|{
        dirs+=1;
        println!("{}", lsr::read_dir(arg.as_ref())?);
        Result::Ok(())
    })?;
    println!("{dirs}");
    if dirs==0 {
        println!("{}", lsr::read_dir(".".as_ref())?);
    }
    Ok(())
}

