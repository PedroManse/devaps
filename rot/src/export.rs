use crate::graph::*;
use crate::RotError;
use std::collections::HashMap;
use std::fmt;

pub(crate) mod rot {
    use super::*;
    pub(crate) fn export(g: &Graph) -> Result<(), RotError> {
        println!("{}", Export(g));
        Ok(())
    }
    struct Export<'a>(pub(crate) &'a Graph);

    impl fmt::Display for Export<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0
                .nodes
                .iter()
                .try_for_each(|n| self.display_node(f, n))?;
            self.0
                .links
                .iter()
                .try_for_each(|n| self.display_link(f, n))
        }
    }

    impl Export<'_> {
        fn display_link(&self, f: &mut fmt::Formatter<'_>, l: &Link) -> fmt::Result {
            let from = &self.0.get_node_by_id(l.from_node_id).unwrap().name;
            let to = &self.0.get_node_by_id(l.to_node_id).unwrap().name;
            write!(f, "{from}->{to}")?;
            self.display_props(f, &l.props)?;
            f.write_str("\n")
        }
        fn display_node(&self, f: &mut fmt::Formatter<'_>, n: &Node) -> fmt::Result {
            f.write_str(&n.name)?;
            self.display_props(f, &n.props)?;
            f.write_str("\n")
        }
        fn display_props(
            &self,
            f: &mut fmt::Formatter<'_>,
            p: &Option<HashMap<String, String>>,
        ) -> fmt::Result {
            if let Some(p) = p {
                f.debug_map().entries(p).finish()
            } else {
                Ok(())
            }
        }
    }
}

pub(crate) mod dot {
    use super::*;
    pub(crate) fn export(g: &Graph) -> Result<(), RotError> {
        println!("{}", Export(g));
        Ok(())
    }

    pub(super) struct Export<'a>(pub(crate) &'a Graph);
    impl fmt::Display for Export<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "digraph RotGraph {{")?;
            self.0
                .nodes
                .iter()
                .try_for_each(|n| self.display_node(f, n))?;
            self.0
                .links
                .iter()
                .try_for_each(|n| self.display_link(f, n))?;
            write!(f, "}}")
        }
    }

    impl Export<'_> {
        fn display_link(&self, f: &mut fmt::Formatter<'_>, l: &Link) -> fmt::Result {
            let from = &self.0.get_node_by_id(l.from_node_id).unwrap().name;
            let to = &self.0.get_node_by_id(l.to_node_id).unwrap().name;
            write!(f, "\t{from}->{to}")?;
            self.display_props(f, &l.props)?;
            f.write_str("\n")
        }
        fn display_node(&self, f: &mut fmt::Formatter<'_>, n: &Node) -> fmt::Result {
            f.write_str("\t")?;
            f.write_str(&n.name)?;
            self.display_props(f, &n.props)?;
            f.write_str("\n")
        }
        fn display_props(
            &self,
            f: &mut fmt::Formatter<'_>,
            p: &Option<HashMap<String, String>>,
        ) -> fmt::Result {
            if let Some(p) = p {
                f.write_str(" [")?;
                p.iter().try_for_each(|(k, v)| write!(f, "{k}={v},"))?;
                f.write_str("]")?;
            };
            Ok(())
        }
    }
}

pub(crate) mod dotex {
    use super::*;
    pub(crate) fn export(graph: &Graph, ex_to: &str) -> Result<(), RotError> {
        use std::io::{self, Write};
        use std::process::{Command, Stdio};
        let dot_script = dot::Export(graph).to_string();
        let mut graphviz = Command::new("dot")
            .arg(format!("-T{}", ex_to))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        graphviz
            .stdin
            .take()
            .ok_or(RotError::MissingStdioError)?
            .write_all(dot_script.as_bytes())?;
        let procout = graphviz.wait_with_output()?;
        io::stdout().write_all(&procout.stdout)?;
        Ok(())
    }
}

pub mod to {
    use super::*;
    pub fn rot(g: &Graph) -> Result<(), RotError> {
        rot::export(g)
    }
    pub fn dot(g: &Graph) -> Result<(), RotError> {
        dot::export(g)
    }
    pub fn dotex(g: &Graph, to: &str) -> Result<(), RotError> {
        dotex::export(g, to)
    }
}
