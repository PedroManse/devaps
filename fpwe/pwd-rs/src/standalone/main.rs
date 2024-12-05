use pwd_rs::*;

fn main() {
    let config = fancy_unwrap(get_config());
    let pwd = fancy_unwrap(std::env::var("PWD").or(Err("Can't find $PWD env variable")));
    let path = config
        .iter()
        .fold(BDir::new(pwd), BDir::edit)
        .into_string();
    print!("{path}");
}
