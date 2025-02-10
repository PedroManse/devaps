use std::path::PathBuf;


fn rev_find_config() -> Option<PathBuf> {
    let mut cwd = std::env::current_dir().unwrap();
    loop {
        let Some(next) = cwd.parent() else {
            break None
        };
        let conf = cwd.clone().join(".todo.conf");
        if conf.exists() {
            break Some(conf)
        }
        cwd = next.to_path_buf();
    }
}

fn find_global_config() -> Option<PathBuf> {
    std::env::var("TODORS_CONFIG").ok().map(PathBuf::from).or(
        directories::ProjectDirs::from("", "", "todo-rs").map(|d|d.config_dir().to_path_buf().join("todo.conf"))
    )
}

fn main() {
    let config_file = rev_find_config().or(find_global_config());
    println!("{config_file:?}");
}
