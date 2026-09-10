/* took from https://github.com/mbrubeck/robinson/blob/master/src/dom.rs */

use std::collections::{HashMap, HashSet};
use crate::layout::LayoutBox;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,

    pub perent: Option<usize>,
    pub first_child: Option<usize>,
    pub next_sibling: Option<usize>
}

#[derive(Debug)]
pub struct Arena {
    pub nodes: Vec<Node>
}

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attrs: AttrMap,
}

#[derive(Debug)]
pub struct LayoutArena {
    pub boxes: Vec<LayoutBox>,
}

pub fn text(data: String) -> Node {
    Node {
        node_type: NodeType::Text(data),
        perent: None,
        first_child: None,
        next_sibling: None,
    }
}

pub fn elem(tag_name: String, attrs: AttrMap) -> Node {
    Node {
        node_type: NodeType::Element(ElementData { tag_name, attrs }),
        perent: None,
        first_child: None,
        next_sibling: None,
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attrs.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attrs.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}