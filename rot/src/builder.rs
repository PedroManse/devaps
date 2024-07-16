use crate::graph::{self, Graph};
use crate::parse2::{self, Item};
use crate::RotError;
use std::collections::HashMap;

fn to_builder_node(mut items: Vec<Item>) -> Result<Vec<BuilderItem>, RotError> {
    items.push(Item::Link); // hack because the last item is ignored
    let items = items.into_iter();
    let mut out = vec![];
    // props as previous item doesn't alter anything
    // so this can be used as 'last' item without special logic
    let mut last = Item::Props(HashMap::new());
    for item in items {
        let next = item.clone();
        match (last, item) {
            (Item::Node(n), Item::Props(p)) => out.push(BuilderItem {
                item: BuilderEntity::Node(n),
                prop: Some(p),
            }),
            (Item::NodeVec(ns), Item::Props(p)) => out.push(BuilderItem {
                item: BuilderEntity::NodeVec(ns),
                prop: Some(p),
            }),
            (Item::Link, Item::Props(p)) => out.push(BuilderItem {
                item: BuilderEntity::Link,
                prop: Some(p),
            }),
            (Item::Props(..), Item::Props(..)) => return Err(RotError::DoubleProp),
            (Item::Props(..), _) => {}
            (Item::Node(n), _) => out.push(BuilderItem {
                item: BuilderEntity::Node(n),
                prop: None,
            }),
            (Item::NodeVec(ns), _) => out.push(BuilderItem {
                item: BuilderEntity::NodeVec(ns),
                prop: None,
            }),
            (Item::Link, _) => out.push(BuilderItem {
                item: BuilderEntity::Link,
                prop: None,
            }),
        };
        last = next;
    }
    Ok(out)
}

#[derive(Debug, Clone)]
enum BuilderEntity {
    NodeVec(Vec<String>),
    Node(String),
    Link,
}

#[derive(Debug, Clone)]
struct BuilderItem {
    item: BuilderEntity,
    prop: Option<HashMap<String, String>>,
}

pub fn build(items: Vec<Item>) -> Result<graph::Graph, RotError> {
    use BuilderEntity as S;
    let items = to_builder_node(items)?;
    let items = items.into_iter();
    let mut graph = graph::Graph::default();
    //// props as previous item doesn't alter anything
    //// so this can be used as 'last' item without special logic
    let mut last: Option<BuilderEntity> = None;
    for item in items {
        let prop = item.prop;
        let item = item.item;
        let this = item.clone();
        match (last, item) {
            (None | Some(S::Node(..)) | Some(S::NodeVec(..)), S::NodeVec(ns)) => {
                ns.into_iter().for_each(|n| {
                    graph.new_node(n, prop.clone());
                })
            }
            (None | Some(S::Node(..)) | Some(S::NodeVec(..)), S::Node(n)) => {
                graph.new_node(n, prop);
            }
        }
        last = Some(this);
    }
    //TODO deal with last
    Ok(graph)
}
