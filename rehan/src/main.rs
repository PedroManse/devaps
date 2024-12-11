use rehan::*;

fn main() -> Result<(), Error> {
    let doc = parse::parse(PathBuf::from("./birthday-email.rehan.txt"))?;
    let vars = HashMap::<String, String>::from([
        ("name".to_string(), "pedro".to_string()),
        ("lastname".to_string(), "Manse".to_string()),
        ("age".to_string(), "19".to_string()),
    ]);
    let doc = doc.format(vars).unwrap();
    println!("FILE: {:?}", doc.file_name);
    println!("{}", doc.content);
    Ok(())
}
