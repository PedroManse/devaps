use crate::RotError;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Item {
    NodeVec(Vec<String>),
    Props(HashMap<String, String>),
    Node(String),
    Link,
}

#[derive(Debug, Clone)]
pub enum Parser {
    Nothing,
    OnNode,
    OnNodeVec,
    OnLink,
    OnProp,
    OnComment,
}

pub fn parse(text: String) -> Result<Vec<Item>, RotError> {
    use Parser as S;
    let mut items = vec![];
    let mut state = S::Nothing;
    let mut buffer = String::new();
    let mut buffer_buffer: Vec<String> = Vec::new();

    for chr in text.chars() {
        match (&state, chr) {
            (S::Nothing, ' ' | '\n' | '\t') => {}
            (S::Nothing, '#') => state = Parser::OnComment,
            (S::OnComment, '\n') => state = Parser::Nothing,
            (S::OnComment, _) => {}
            (S::Nothing, '[') => {
                state = Parser::OnNodeVec;
            }
            (S::OnNodeVec, ',') => {
                buffer_buffer.push(buffer);
                buffer = String::new();
            }
            (S::OnNodeVec, ']') => {
                if !buffer.is_empty() {
                    buffer_buffer.push(buffer);
                }
                items.push(Item::NodeVec(buffer_buffer));
                state = Parser::Nothing;
                buffer = String::new();
                buffer_buffer = Vec::new();
            }
            (S::Nothing, '-') => {
                state = Parser::OnLink;
            }
            (S::OnNode, '-') => {
                items.push(Item::Node(buffer));
                state = Parser::OnLink;
                buffer = String::new();
            }
            (S::OnLink, '>') => {
                items.push(Item::Link);
                state = Parser::Nothing;
            }
            (S::OnNode, '\n') => {
                items.push(Item::Node(buffer));
                state = Parser::Nothing;
                buffer = String::new();
            }
            (S::OnNode, '{') => {
                items.push(Item::Node(buffer));
                state = Parser::OnProp;
                buffer = String::new();
            }
            (S::OnProp, '}') => {
                items.push(Item::Props(prop_to_hashmap(buffer)?));
                state = Parser::Nothing;
                buffer = String::new();
            }
            (S::OnNodeVec, c @ '{') => return Err(RotError::IlegalCharName(c, buffer)),
            (S::OnNode, c @ ',') => return Err(RotError::IlegalCharName(c, buffer)),
            (S::OnLink, chr) => {
                return Err(RotError::LinkSyntaxError(chr));
            }
            (S::Nothing, chr) => {
                state = Parser::OnNode;
                buffer.push(chr);
            }
            //TODO OnProp should make OnProp use prop_to_hashmap without buffer
            // prop_to_hashmap uses a char iterator anyway
            (S::OnNodeVec | S::OnNode | S::OnProp, chr) => buffer.push(chr),
        }
        //println!("{state:?}");
    }

    match state {
        Parser::Nothing => Ok(items),
        Parser::OnNode => Ok(items),
        a => Err(RotError::UnclosedState(a)),
    }
}

enum PropParser {
    OnKey,
    OnValue,
    ShouldValue,
    ShouldKey,
}

fn prop_to_hashmap(prop: String) -> Result<HashMap<String, String>, RotError> {
    use PropParser as S;
    let mut items = HashMap::new();
    let mut state = S::ShouldKey;
    let mut buffer = String::new();
    let mut key_buffer = String::new();

    for chr in prop.chars() {
        match (&state, chr) {
            (S::OnKey, ':') => {
                key_buffer = buffer;
                buffer = String::new();
                state = S::ShouldValue;
            }
            (S::ShouldValue | S::ShouldKey, ' ' | '\t' | '\n') => {}
            (S::ShouldValue, '"') => {
                state = S::OnValue;
            }
            (S::ShouldValue, chr) => return Err(RotError::DidntStartValue(buffer, chr)),
            (S::OnValue, '"') => {
                items.insert(key_buffer, buffer);
                buffer = String::new();
                key_buffer = String::new();
                state = S::ShouldKey;
            }
            (S::ShouldKey, ',') => {}
            (S::ShouldKey, chr) => {
                buffer.push(chr);
                state = S::OnKey;
            }
            (S::OnKey | S::OnValue, chr) => buffer.push(chr),
        }
    }

    match (
        !key_buffer.is_empty(),
        key_buffer,
        !buffer.is_empty(),
        buffer,
    ) {
        (true, k, true, v) => {
            items.insert(k, v);
        }
        (true, k, false, _) => return Err(RotError::KeyWithoutValue(k)),
        (false, _, true, v) => return Err(RotError::ValueWithoutKey(v)),
        (false, _, false, _) => {}
    }
    Ok(items)
}
