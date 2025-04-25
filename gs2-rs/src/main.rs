use gs2::*;

fn main() {
    let repo = match git2::Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return
    };
    if repo.is_bare() {
        print!(" \x1b[1;4;35m<bare>\x1b[0m ");
        return;
    }

    // get branch name, remote
    let mut status_reporter = match StatusReport::new(&repo) {
        Ok(s)=>s,
        Err(e)=>{
            eprintln!("{e}");
            return
        }
    };
    // get diffs
    if let Err(e) = status_reporter.update_statuses() {
        eprintln!("{e}");
        return
    };
    // get remote graph
    if let Err(e) = status_reporter.update_graph() {
        eprintln!("{e}");
        return
    };
    print!("{status_reporter}");
}

