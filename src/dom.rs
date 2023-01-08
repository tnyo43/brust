use std::collections::HashMap;

pub type AttributeMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
struct ElementData {
    tag_name: String,
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
