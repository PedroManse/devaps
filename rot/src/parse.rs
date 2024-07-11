use crate::RotError;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    name: String,
    props: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Link {
    from: String,
    to: String,
    props: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Item {
    Node(Node),
    Link(Link),
}

impl Item {
    fn node(&self) -> Option<&Node> {
        match self {
            Item::Node(n)=>Some(n),
            _=>None,
        }
    }
}

impl From<Node> for Item {
    fn from(n: Node) -> Item {
        Item::Node(n)
    }
}
impl From<Link> for Item {
    fn from(l: Link) -> Item {
        Item::Link(l)
    }
}

pub fn parse(text: String) -> Result<Vec<Item>, RotError> {
    let mut chars = text.chars().peekable();
    let mut items = vec![];
    while let Some(item) = next(&mut chars) {
        items.append(&mut item?);
    }
    Ok(items)
}

fn next(chars: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Option<Result<Vec<Item>, RotError>> {
    chars.peek()?;
    Some(node(chars))
}

fn node(chars: &mut impl Iterator<Item = char>) -> Result<Vec<Item>, RotError> {
    let mut name = String::new();
    while let Some(chr) = chars.next() {
        match chr {
            '\n' => {
                return Ok(vec![Node {
                    name,
                    props: HashMap::new(),
                }
                .into()])
            }
            '{' => {
                let props = props(chars)?;
                return Ok(vec![Node { name, props }.into()]);
            }
            '[' | ']' => {
                return Err(RotError::IlegalCharName(chr, name));
            }
            '-' => {
                let link = link(name.clone(), chars)?;
                let node = Node {
                    name,
                    props: HashMap::new(),
                }
                .into();
                return Ok(vec![vec![node], link].into_iter().flatten().collect());
            }
            x => {
                name.push(x);
            }
        }
    }
    Err(RotError::MissingInfo(name))
}

fn link(node_from: String, chars: &mut impl Iterator<Item = char>) -> Result<Vec<Item>, RotError> {
    assert!(chars.next() == Some('>'));
    let items = node(chars)?;
    let node_to = items[0].node().unwrap().name.clone();
    let link: Item = Link{to: node_to, from: node_from, props: HashMap::new()}.into();
    return Ok(vec![vec![link], items].into_iter().flatten().collect());
}

fn props(chars: &mut impl Iterator<Item = char>) -> Result<HashMap<String, String>, RotError> {
    let mut str_buffer = String::new();
    let mut props = HashMap::<String, String>::new();
    todo!()
}
