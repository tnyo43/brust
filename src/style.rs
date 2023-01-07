use std::clone;

#[derive(Debug, PartialEq)]
pub struct Selector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn new(tag: Option<String>, id: Option<String>, class: Vec<String>) -> Self {
        Selector {
            tag: tag,
            id: id,
            class: class,
        }
    }

    pub fn specificity(&self) -> Specificity {
        (
            self.id.iter().count(),
            self.class.len(),
            self.tag.iter().count(),
        )
    }
}

impl Value {
    pub fn clone(&self) -> Self {
        match self {
            Value::String(s) => Value::String(s.clone()),
        }
    }
}

impl Declaration {
    pub fn new(name: String, value: String) -> Self {
        Declaration {
            name: name,
            value: Value::String(value),
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

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use rstest::*;
    use speculate::speculate;

    use super::*;

    speculate! {
        describe "calculate specificity" {
            describe "first value represents if a id is specified" {
                #[rstest]
                fn is_0_if_id_is_not_specified() {
                    let selector = Selector::new(None, None, Vec::new());
                    assert_eq!(selector.specificity().0, 0)
                }

                #[rstest]
                fn is_1_if_id_is_specified() {
                    let selector = Selector::new(None, Some("id".to_string()), Vec::new());
                    assert_eq!(selector.specificity().0, 1)
                }
            }

            describe "second value represents how many classes is specified" {
                #[rstest(classes,
                    case(Vec::new()),
                    case(Vec::from(["a", "b", "c"])),
                )]
                fn is_length_of_classes(classes: Vec<&str>) {
                    let selector = Selector::new(None, Some("id".to_string()), classes.iter().map(|c| c.to_string()).collect());
                    assert_eq!(selector.specificity().1, classes.len())
                }
            }

            describe "third value represent if a tag name is specified" {
                #[rstest]
                fn is_0_if_tag_name_is_not_specified() {
                    let selector = Selector::new(None, None, Vec::new());
                    assert_eq!(selector.specificity().2, 0)
                }

                #[rstest]
                fn is_1_if_tag_name_is_specified() {
                    let selector = Selector::new(Some("tag".to_string()), None, Vec::new());
                    assert_eq!(selector.specificity().2, 1)
                }
            }
        }
    }
}
