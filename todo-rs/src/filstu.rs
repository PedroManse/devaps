use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum Node<A, H> {
    Atom(A),
    List(H, Vec<Node<A, H>>),
}
pub type Entry = Node<PathBuf, PathBuf>;

impl<Ai, Hi> Node<Ai, Hi> {
    pub fn map<Ao, Ho, Ff, Fd>(self, mut dir_cb: Fd, mut file_cb: Ff) -> Node<Ao, Ho>
    where
        Ff: FnMut(Ai) -> Ao,
        Fd: FnMut(Hi, &[Node<Ai, Hi>]) -> Ho,
    {
        self.map_(&mut dir_cb, &mut file_cb)
    }
    
    fn map_<Ao, Ho, Ff, Fd>(self, dir_cb: &mut Fd, file_cb: &mut Ff) -> Node<Ao, Ho>
    where
        Ff: FnMut(Ai) -> Ao,
        Fd: FnMut(Hi, &[Node<Ai, Hi>]) -> Ho,
    {
        match self {
            Node::Atom(f)=>Node::Atom(file_cb(f)),
            Node::List(f, fs)=>{
                Node::List(
                    dir_cb(f, &fs),
                    fs.into_iter().map(|f|f.map_(dir_cb, file_cb)).collect()
                )
            },
        }
    }

    pub fn atom(&self) -> Option<&Ai> {
        match self {
            Node::Atom(a)=>Some(a),
            Node::List(..)=>None,
        }
    }
    pub fn list(&self) -> Option<(&Hi, &[Self])> {
        match self {
            Node::Atom(..)=>None,
            Node::List(h, xs)=>Some((h, &xs)),
        }
    }
}

pub fn read_dir(dir: &Path) -> Result<Entry, std::io::Error> {
    if !dir.is_dir() {
        return Ok(Entry::Atom(dir.into()));
    }
    let out: Result<Vec<_>, _> = fs::read_dir(dir)?
        .map(|f| read_dir(&f?.path()))
        .collect();
    Ok(Entry::List(dir.to_owned(), out?))
}

pub fn read_dir_fitered<F>(
    dir: &Path,
    filter: &F,
) -> Result<Entry, std::io::Error>
where
    F: PathFilter,
{
    if !dir.is_dir() {
        if filter.filter_dir(&dir) {
            return Ok(Entry::Atom(dir.into()));
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Can't read_dir_fitered a file"));
        }
    }
    let out: Result<Vec<_>, _> = fs::read_dir(dir)?
        .filter(|f|{
            f.as_ref().map(|x|{
                let path = x.path();
                match path.is_dir() {
                    true=>filter.filter_dir(path.as_path()),
                    false=>filter.filter_file(path.as_path()),
                }
            }).unwrap_or(false)
        })
        .map(|f| read_dir_fitered(&f?.path(), filter))
        .collect();
    Ok(Entry::List(dir.to_owned(), out?))
}

pub trait PathFilter {
    fn filter_file(&self, p: &Path) -> bool;
    fn filter_dir(&self, p: &Path) -> bool;
}
