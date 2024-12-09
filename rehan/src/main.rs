use rehan::*;

fn main() {
    let x = RawDocument{
        file_name: std::path::PathBuf::from("birthday-email.rehan.txt"),
        directives: vec![
            Directive::Input{
                name: "name".to_string(),
                transforms: vec![Transform::UpperCaseFirst],
            },
            Directive::Input{
                name: "lastname".to_string(),
                transforms: vec![Transform::UpperCaseFirst],
            },
            Directive::Input{
                name: "age".to_string(),
                transforms: vec![Transform::IsInt, Transform::IsGreaterThan(0.0)],
            },
            Directive::Filename{
                expr: "birthday-email-{name} {lastname}-{age}.txt".to_string(),
            },
            Directive::Format{
                name: "full_name".to_string(),
                expr: "{name} {lastname}".to_string(),
            },
            Directive::Set{
                name: "FULLNAME".to_string(),
                from: "full_name".to_string(),
                transforms: vec![Transform::AllUpperCase],
            }
        ],
        actual_content: r#"
Hello {name}!

Congrats on your {age}th birthday!

- From me
- To {FULLNAME}
        "#.to_string(),
    };
    let vars = HashMap::<String, String>::from([
        ("name".to_string(), "Pedro".to_string()),
        ("2".to_string(), "Manse".to_string()),
        ("3".to_string(), "19".to_string()),
    ]);
    let doc = x.format(vars).unwrap();
    println!("FILE: {:?}", doc.file_name);
    println!("{}", doc.content);
}
