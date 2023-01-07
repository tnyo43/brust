use std::collections::HashMap;

type AttributeMap = HashMap<String, String>;

struct ElementData {
    tag_name: String,
    attributes: AttributeMap,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}
