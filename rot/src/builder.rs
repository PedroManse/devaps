use crate::graph;
use crate::parse2::Item;
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

#[derive(Clone, Debug)]
enum BuilderEntity {
    NodeVec(Vec<String>),
    Node(String),
    Link,
}

#[derive(Debug)]
struct BuilderItem {
    item: BuilderEntity,
    prop: Option<HashMap<String, String>>,
}

#[derive(Debug)]
enum BuilderState {
    Nothing,
    ShouldLinkNode(String, Option<HashMap<String, String>>),
    ShouldLinkNodeVec(Vec<String>, Option<HashMap<String, String>>),
}

pub fn build(graph: &mut graph::Graph, items: Vec<Item>) -> Result<(), RotError> {
    use BuilderEntity as S;
    use BuilderState::Nothing as SDef;
    let mut state = BuilderState::Nothing;
    let items = to_builder_node(items)?;
    let items = items.into_iter();
    let mut last: Option<BuilderEntity> = None;
    for item in items {
        let prop = item.prop;
        let item = item.item;
        let this = item.clone();
        match (&state, last, item) {
            (SDef, None | Some(S::Node(..)) | Some(S::NodeVec(..)), S::NodeVec(ns)) => {
                ns.into_iter()
                    .map(|n| {
                        let node = graph.make_or_get_node_mut(&n)?;
                        if let Some(prop) = prop.clone() {
                            node.extend(prop);
                        }
                        Ok(())
                    })
                    .collect::<Result<Vec<_>, RotError>>()?;
            }
            (SDef, None | Some(S::Node(..)) | Some(S::NodeVec(..)), S::Node(n)) => {
                let node = graph.make_or_get_node_mut(&n)?;
                if let Some(prop) = prop.clone() {
                    node.extend(prop);
                }
            }
            (SDef, None | Some(S::Link), S::Link) => {
                Err(RotError::DoubleLink)?;
            }
            (
                BuilderState::ShouldLinkNode(..) | BuilderState::ShouldLinkNodeVec(..),
                Some(S::Link),
                S::Link,
            ) => {
                Err(RotError::DoubleLink)?;
            }
            (SDef, Some(S::Node(n)), S::Link) => {
                state = BuilderState::ShouldLinkNode(n, prop);
            }
            (BuilderState::ShouldLinkNode(from, link_prop), Some(S::Link), S::Node(to)) => {
                let from_id = graph.get_id_by_name(from)?;
                let to_id = {
                    let to_node = graph.make_or_get_node_mut(&to)?;
                    if let Some(prop) = prop {
                        to_node.extend(prop);
                    }
                    to_node.id
                };
                graph.link_nodes(from_id, to_id, link_prop.clone())?;
                state = SDef;
            }
            (BuilderState::ShouldLinkNode(from, link_prop), Some(S::Link), S::NodeVec(tos)) => {
                let from_id = graph.get_id_by_name(from)?;
                let to_ids: Vec<_> = tos
                    .into_iter()
                    .map(|n| {
                        let to_node = graph.make_or_get_node_mut(n)?;
                        if let Some(prop) = prop.clone() {
                            to_node.extend(prop);
                        }
                        Ok(to_node.id)
                    })
                    .collect::<Result<_, RotError>>()?;
                for to_id in to_ids {
                    graph.link_nodes(from_id, to_id, link_prop.clone())?;
                }
                state = SDef;
            }
            (SDef, Some(S::NodeVec(n)), S::Link) => {
                state = BuilderState::ShouldLinkNodeVec(n, prop);
            }
            (BuilderState::ShouldLinkNodeVec(froms, link_prop), Some(S::Link), S::Node(to)) => {
                let from_ids: Vec<_> = froms
                    .iter()
                    .map(|n| graph.get_id_by_name(n))
                    .collect::<Result<_, _>>()?;
                let to_id = graph
                    .get_id_by_name(&to)
                    .or_else(|_| graph.new_node(to, prop.clone()).map(|nn| nn.id))?;
                for from_id in from_ids {
                    graph.link_nodes(from_id, to_id, link_prop.clone())?;
                }
                state = SDef;
            }
            (BuilderState::ShouldLinkNodeVec(froms, link_prop), Some(S::Link), S::NodeVec(tos)) => {
                let from_ids: Vec<_> = froms
                    .iter()
                    .map(|n| graph.get_id_by_name(n))
                    .collect::<Result<_, _>>()?;
                let to_ids: Vec<_> = tos
                    .iter()
                    .map(|n| graph.get_id_by_name(n))
                    .collect::<Result<_, _>>()?;
                //TODO implitic node; use prop
                for from_id in from_ids {
                    for to_id in to_ids.iter() {
                        graph.link_nodes(from_id, *to_id, link_prop.clone())?;
                    }
                }
                state = SDef;
            }
            (
                BuilderState::ShouldLinkNodeVec(..) | BuilderState::ShouldLinkNode(..),
                None | Some(S::Node(..)) | Some(S::NodeVec(..)),
                _,
            ) => {
                unreachable!()
            }
            (SDef, Some(S::Link), _) => {
                unreachable!()
            }
        }
        last = Some(this);
    }
    Ok(())
}
