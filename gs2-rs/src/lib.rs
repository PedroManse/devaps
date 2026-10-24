use git2::Error as GErr;
use git2::*;
type GitResult<T> = Result<T, GErr>;

fn get_remote_name(repo: &'_ Repository) -> Option<String> {
    repo.remotes()
        .ok()?
        .iter()
        .flatten()
        .flatten()
        .next()
        .map(|n| n.to_owned())
}

pub struct StatusReport<'r> {
    repo: &'r Repository,
    staged: bool,
    new: bool,
    modified: bool,
    deleted: bool,
    renamed: bool,
    conflict: bool,
    branch: String,
    remote: Option<String>,
    diverge: bool,
}

fn get_branch_name(repo: &Repository) -> String {
    repo.head()
        .ok()
        .and_then(|h| h.shorthand().ok().map(str::to_owned))
        .unwrap_or_else(|| "<HEADLESS>".to_owned())
}

impl<'r> StatusReport<'r> {
    pub fn new(repo: &'r Repository) -> GitResult<Self> {
        let branch = get_branch_name(repo);
        let remote = get_remote_name(repo);
        Ok(StatusReport {
            repo,
            staged: false,   // →
            new: false,      // +
            modified: false, // *
            deleted: false,  // -
            renamed: false,  // ←
            conflict: false, // x
            branch,          // {name}
            diverge: false,  // ↕
            remote,
        })
    }

    pub fn update_graph(&mut self) -> Result<(), GErr> {
        self.diverge = self.get_graph()?;
        Ok(())
    }

    pub fn get_graph(&self) -> GitResult<bool> {
        let Some(remote) = &self.remote else {
            return Ok(false);
        };
        let local = self.repo.head()?;
        let (Some(local_target), Ok(local_sname)) = (local.target(), local.shorthand()) else {
            return Ok(false);
        };
        let remote_ref = format!("refs/remotes/{remote}/{local_sname}");
        match self.repo.find_reference(&remote_ref) {
            Ok(r) => Ok(r.target() != Some(local_target)),
            Err(e) if e.code() == ErrorCode::NotFound => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub fn update_statuses(&mut self) -> Result<(), GErr> {
        let mut opts = StatusOptions::new();
        opts.include_ignored(false)
            .include_untracked(true)
            .recurse_untracked_dirs(false)
            .exclude_submodules(true)
            .update_index(false);
        let statuses = self.repo.statuses(Some(&mut opts))?;
        for entry in &statuses {
            self.update(entry.status());
        }
        Ok(())
    }

    fn update(&mut self, status: Status) {
        let is_index_new = status.is_index_new();
        let is_index_modified = status.is_index_modified();
        let is_index_deleted = status.is_index_deleted();
        let is_index_renamed = status.is_index_renamed();
        let is_index_typechange = status.is_index_typechange();
        let is_wt_new = status.is_wt_new();
        let is_wt_modified = status.is_wt_modified();
        let is_wt_deleted = status.is_wt_deleted();
        //let is_wt_typechange = status.is_wt_typechange();
        let is_wt_renamed = status.is_wt_renamed();
        let is_conflicted = status.is_conflicted();
        self.staged |= is_index_new
            | is_index_modified
            | is_index_deleted
            | is_index_renamed
            | is_index_typechange;
        self.new |= is_wt_new;
        self.modified |= is_wt_modified;
        self.deleted |= is_wt_deleted;
        self.renamed |= is_wt_renamed;
        self.conflict |= is_conflicted;
    }
}

impl<'r> std::fmt::Display for StatusReport<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let clean = !(self.staged
            | self.new
            | self.modified
            | self.deleted
            | self.renamed
            | self.diverge
            | self.conflict);
        write!(f, " \x1b[1;4;35m{}\x1b[0m ", self.branch)?;
        if clean {
            write!(f, "\x1b[32m✓")?
        };
        if self.new {
            write!(f, "\x1b[31m+")?
        };
        if self.staged {
            write!(f, "\x1b[31m→")?
        };
        if self.conflict {
            write!(f, "\x1b[5;31mx")?
        };
        if self.deleted {
            write!(f, "\x1b[31m-")?
        };
        if self.renamed {
            write!(f, "\x1b[34m←")?
        };
        if self.modified {
            write!(f, "\x1b[31m*")?
        };
        if self.diverge {
            write!(f, "\x1b[7;5;31m↕")?;
        }
        write!(f, "\x1b[0m")
    }
}
