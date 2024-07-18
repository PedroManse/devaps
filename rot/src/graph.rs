use crate::RotError;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Default)]
pub struct Graph {
    node_count: AtomicUsize,
    link_count: AtomicUsize,
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
    pub(crate) props: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Node {
    #[allow(dead_code)]
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) props: Option<HashMap<String, String>>,
    pub(crate) links: HashSet<usize>,
    pub(crate) back_links: HashSet<usize>,
}

impl Node {
    pub fn extend(&mut self, props: HashMap<String, String>) {
        self.props = Some(if let Some(mut old_props) = self.props.clone() {
            old_props.extend(props);
            old_props
        } else {
            props
        });
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph::default()
    }
    fn count_node(&self) -> usize {
        self.node_count.fetch_add(1, Ordering::SeqCst)
    }
    fn count_link(&self) -> usize {
        self.link_count.fetch_add(1, Ordering::SeqCst)
    }
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
    pub fn extend_prop(
        &mut self,
        node_id: usize,
        prop: HashMap<String, String>,
    ) -> Result<&Node, RotError> {
        let node = self.get_node_by_id_mut(node_id)?;
        node.extend(prop);
        Ok(node)
    }
    pub fn link_nodes(
        &mut self,
        from_node_id: usize,
        to_node_id: usize,
        props: Option<HashMap<String, String>>,
    ) -> Result<&Link, RotError> {
        let id = self.count_link();
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

    pub fn make_or_get_node_mut<S>(&mut self, name: S) -> Result<&mut Node, RotError>
    where
        S: Into<String>,
    {
        let name = name.into();
        let maybe_id = self.get_id_by_name(&name);
        match maybe_id {
            Ok(id) => Ok(&mut self.nodes[id]),
            Err(_) => self.new_node(name, None),
        }
    }
    pub fn new_node<S>(
        &mut self,
        name: S,
        props: Option<HashMap<String, String>>,
    ) -> Result<&mut Node, RotError>
    where
        S: Into<String>,
    {
        let name = name.into();
        if self.nodes_by_name.contains_key(&name) {
            return Err(RotError::NodeOverwrite(name));
        }
        let id = self.count_node();
        self.nodes_by_name.insert(name.clone(), id);
        let n = Node {
            id,
            name,
            props,
            links: HashSet::new(),
            back_links: HashSet::new(),
        };
        self.nodes.push(n);
        Ok(&mut self.nodes[id])
    }
}
