use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum Node<A, H> {
    Atom(A),
    List(H, Vec<Node<A, H>>),
}
pub struct FPath(pub PathBuf);
#[derive(Debug)]
pub struct DPath(pub PathBuf);
pub type Entry = Node<FPath, DPath>;

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
            Node::Atom(f) => Node::Atom(file_cb(f)),
            Node::List(f, fs) => Node::List(
                dir_cb(f, &fs),
                fs.into_iter().map(|f| f.map_(dir_cb, file_cb)).collect(),
            ),
        }
    }

    pub fn atom(&self) -> Option<&Ai> {
        match self {
            Node::Atom(a) => Some(a),
            Node::List(..) => None,
        }
    }
    pub fn list(&self) -> Option<(&Hi, &[Self])> {
        match self {
            Node::Atom(..) => None,
            Node::List(h, xs) => Some((h, &xs)),
        }
    }
}

pub fn read_dir_fitered<F, P>(dir: P, filter: &F) -> Result<Entry, std::io::Error>
where
    F: PathFilter,
    P: AsRef<Path>,
{
    let dir = dir.as_ref();
    if !dir.is_dir() {
        if filter.filter_dir(dir) {
            return Ok(Entry::Atom(FPath(dir.into())));
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Can't read_dir_fitered a file",
            ));
        }
    }
    let out: Result<Vec<_>, _> = fs::read_dir(dir)?
        .filter(|f| {
            f.as_ref()
                .map(|x| {
                    let path = x.path();
                    match path.is_dir() {
                        true => filter.filter_dir(path.as_path()),
                        false => filter.filter_file(path.as_path()),
                    }
                })
                .unwrap_or(false)
        })
        .map(|f| read_dir_fitered(&f?.path(), filter))
        .collect();
    Ok(Entry::List(DPath(dir.to_owned()), out?))
}

pub trait PathFilter {
    fn filter_file(&self, p: &Path) -> bool;
    fn filter_dir(&self, p: &Path) -> bool;
}

mod display {
    use super::*;
    use std::fmt::{self, Display};
    impl Display for FPath {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = self.0.file_name().unwrap().to_str().unwrap();
            f.write_str(name)
        }
    }
    impl Display for DPath {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = self.0.file_name().and_then(|f| f.to_str()).unwrap_or(".");
            f.write_str(name)
        }
    }

    impl<Na: fmt::Display, Nb: fmt::Display> fmt::Display for Node<Na, Nb> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            display_entry(f, self, "", "")
        }
    }

    fn display_entry<Na, Nb>(
        f: &mut fmt::Formatter<'_>,
        e: &Node<Na, Nb>,
        pad: &str,
        segment: &str,
    ) -> Result<(), fmt::Error>
    where
        Node<Na, Nb>: Display,
        Na: Display,
        Nb: Display,
    {
        f.write_str(pad)?;
        match e {
            Node::Atom(name) => {
                let name = format!("{}", name);
                f.write_str(&name)?;
                f.write_str("\n")?;
            }
            Node::List(name, fs) => {
                let name = format!("{}", name);
                f.write_str(&name)?;
                f.write_str("\n")?;
                let nextpad = String::from(segment) + "├──";
                let lastpad = String::from(segment) + "└──";
                let nextseg = String::from(segment) + "│  ";
                let lastseg = String::from(segment) + "   ";
                for (idx, fl) in fs.iter().enumerate() {
                    if idx == fs.len() - 1 {
                        display_entry(f, fl, &lastpad, &lastseg)?;
                    } else {
                        display_entry(f, fl, &nextpad, &nextseg)?;
                    }
                }
            }
        };
        Ok(())
    }
}
