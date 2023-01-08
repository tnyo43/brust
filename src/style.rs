#[derive(Debug, PartialEq)]
pub struct Selector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    name: String,
    value: String,
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub struct StyleSheet {
    rules: Vec<Rule>,
}

impl Selector {
    pub fn new(tag: Option<String>, id: Option<String>, class: Vec<String>) -> Self {
        Selector {
            tag: tag,
            id: id,
            class: class,
        }
    }
}

impl Declaration {
    pub fn new(name: String, value: String) -> Self {
        Declaration {
            name: name,
            value: value,
        }
    }
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Self {
        Rule {
            selectors: selectors,
            declarations: declarations,
        }
    }
}

impl StyleSheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        StyleSheet { rules: rules }
    }
}
