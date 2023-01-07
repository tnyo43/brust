use std::collections::{HashMap, HashSet};

pub type AttributeMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    attributes: AttributeMap,
}

#[derive(Debug, PartialEq)]
enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

impl ElementData {
    pub fn new(name: String, attributes: AttributeMap) -> Self {
        ElementData {
            tag_name: name,
            attributes: attributes,
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classes) => classes.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

impl Node {
    pub fn text(data: String) -> Self {
        Node {
            children: vec![],
            node_type: NodeType::Text(data),
        }
    }

    pub fn element(name: String, attributes: AttributeMap, children: Vec<Node>) -> Self {
        Node {
            children: children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attributes,
            }),
        }
    }
}
