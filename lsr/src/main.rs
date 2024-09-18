use std::io;

fn main() -> Result<(), io::Error> {
    let args = std::env::args();
    args.skip(1).into_iter().try_fold((), |_, arg|{
        let nodes = lsr::read_dir(arg.as_ref())?;
        println!("{nodes}");
        Ok::<(), io::Error>(())
    })
}

