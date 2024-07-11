use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::RotError;

static NODE_COUNT: AtomicUsize = AtomicUsize::new(0);
fn count_node() -> usize {
    NODE_COUNT.fetch_add(1, Ordering::SeqCst)
}
static LINK_COUNT: AtomicUsize = AtomicUsize::new(0);
fn count_link() -> usize {
    LINK_COUNT.fetch_add(1, Ordering::SeqCst)
}

#[derive(Debug, Default)]
pub struct Graph {
    pub(crate) nodes: Vec<Node>,
    pub(crate) links: Vec<Link>,
    pub(crate) nodes_by_name: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct Link {
    #[allow(dead_code)]
    pub(crate) id: usize,
    pub(crate) from_node_id: usize,
    pub(crate) to_node_id: usize,
    pub(crate) props: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Node {
    #[allow(dead_code)]
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) props: HashMap<String, String>,
    pub(crate) links: HashSet<usize>,
    pub(crate) back_links: HashSet<usize>,
}

impl Graph {
    pub fn get_id_by_name(&mut self, name: &str) -> Result<usize, RotError> {
        self.nodes_by_name
            .get(name)
            .copied()
            .ok_or(RotError::NoNodeName(name.to_owned()))
    }
    pub fn get_node_by_id(&self, id: usize) -> Result<&Node, RotError> {
        if id >= self.nodes.len() {
            Err(RotError::NoNodeId(id))
        } else {
            Ok(&self.nodes[id])
        }
    }
    pub fn get_link_by_id(&self, id: usize) -> Result<&Link, RotError> {
        if id >= self.links.len() {
            Err(RotError::NoLinkId(id))
        } else {
            Ok(&self.links[id])
        }
    }
    pub fn get_node_by_id_mut(&mut self, id: usize) -> Result<&mut Node, RotError> {
        if id >= self.nodes.len() {
            Err(RotError::NoNodeId(id))
        } else {
            Ok(&mut self.nodes[id])
        }
    }
    pub fn get_link_by_id_mut(&mut self, id: usize) -> Result<&mut Link, RotError> {
        if id >= self.links.len() {
            Err(RotError::NoLinkId(id))
        } else {
            Ok(&mut self.links[id])
        }
    }
    pub fn link_nodes(
        &mut self,
        from_node_id: usize,
        to_node_id: usize,
        props: HashMap<String, String>,
    ) -> Result<&Link, RotError> {
        let id = count_link();
        let l = Link {
            id,
            from_node_id,
            to_node_id,
            props,
        };
        self.links.push(l);
        self.get_node_by_id_mut(from_node_id)?.links.insert(id);
        self.get_node_by_id_mut(to_node_id)?.back_links.insert(id);
        Ok(&self.links[id])
    }

    pub fn new_node<S>(&mut self, name: S, props: HashMap<String, String>) -> &Node
    where
        S: Into<String>,
    {
        let name = name.into();
        let id = count_node();
        self.nodes_by_name.insert(name.clone(), id);
        let n = Node {
            id,
            name: name,
            props,
            links: HashSet::new(),
            back_links: HashSet::new(),
        };
        self.nodes.push(n);
        &self.nodes[id]
    }
}
