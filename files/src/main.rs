use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: many files in one line
    let r = regex::Regex::new(r#"[^ !$`&*()+]+\.[^ !$`&*()+]+"#).unwrap();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        if let Some(s) = r.captures(&line) {
            println!("{}", &s[0]);
        }
    }
    Ok(())
}
