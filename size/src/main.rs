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
    if count > 10*1024*1024*1024 {
        println!("{:.02}Gib", count as f64 / 1024.0 / 1024.0 / 1024.0);
    } else if count > 10*1024*1024 {
        println!("{:.02}Mib", count as f64 / 1024.0 / 1024.0);
    } else if count > 10*1024 {
        println!("{:.02}Kib", count as f64 / 1024.0);
    } else {
        println!("{count}b");
    }
    Ok(())
}
