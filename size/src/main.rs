fn main() -> Result<(), std::io::Error> {
    let args = std::env::args().skip(1);
    let mut count = 0;
    for dir in args {
        let dir = size::read_dir(std::path::Path::new(&dir))?;
        dir.map(
            |_, _|{()},
            |f|{
                f.metadata().map(|m|{
                    count+=m.len();
                })
            }
        );
    }
    println!("{count}");
    Ok(())
}
