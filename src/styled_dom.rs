use std::collections::HashMap;

use crate::dom::{ElementData, Node, NodeType};
use crate::style::{Rule, Selector, Specificity, StyleSheet, Value};

type MatchedRule<'a> = (Specificity, &'a Rule);

type PropertyMap = HashMap<String, Value>;

pub struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

fn matches_selector(element_data: &ElementData, selector: &Selector) -> bool {
    if selector.tag.iter().any(|tag| element_data.tag_name != *tag) {
        return false;
    }

    if selector.id.iter().any(|id| element_data.id() != Some(id)) {
        return false;
    }

    let element_classes = element_data.classes();
    if selector
        .class
        .iter()
        .any(|class| !element_classes.contains(&**class))
    {
        return false;
    }

    true
}

fn matching_rule<'a>(element_data: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches_selector(element_data, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(
    element_data: &ElementData,
    stylesheet: &'a StyleSheet,
) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| matching_rule(element_data, rule))
        .collect()
}

fn specified_values(element_data: &ElementData, stylesheet: &StyleSheet) -> PropertyMap {
    let mut property_map = PropertyMap::new();

    let mut rules = matching_rules(element_data, stylesheet);
    rules.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (_, rule) in rules {
        for declaration in &rule.declarations {
            property_map.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    property_map
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a StyleSheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref element_data) => specified_values(element_data, stylesheet),
            NodeType::Text(_) => HashMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use rstest::*;
    use speculate::speculate;

    use super::*;
    use crate::css;
    use crate::dom::AttributeMap;
    use crate::style::Declaration;

    speculate! {
        describe "'matches_selector'" {
            describe "if tag name is specified" {
                #[rstest]
                fn true_if_tag_name_matches() {
                    let element_data = ElementData::new("hoge".to_string(), AttributeMap::new());
                    let selector = Selector::new(Some("hoge".to_string()), None, Vec::new());

                    assert!(matches_selector(&element_data, &selector));
                }

                #[rstest]
                fn false_if_tag_name_doesnt_match() {
                    let element_data = ElementData::new("div".to_string(), AttributeMap::new());
                    let selector = Selector::new(Some("image".to_string()), None, Vec::new());

                    assert!(!matches_selector(&element_data, &selector));
                }
            }

            describe "if id is specified" {
                #[rstest]
                fn false_if_element_id_is_not_set() {
                    let element_data = ElementData::new("button".to_string(), AttributeMap::new());
                    let selector = Selector::new(None, Some("submit".to_string()), Vec::new());

                    assert!(!matches_selector(&element_data, &selector));
                }

                #[rstest]
                fn false_if_element_id_doesnt_match() {
                    let element_data = ElementData::new("button".to_string(), AttributeMap::from([("id".to_string(), "delete".to_string())]));
                    let selector = Selector::new(None, Some("submit".to_string()), Vec::new());

                    assert!(!matches_selector(&element_data, &selector));
                }

                #[rstest]
                fn true_if_element_id_match() {
                    let element_data = ElementData::new("button".to_string(), AttributeMap::from([("id".to_string(), "submit".to_string())]));
                    let selector = Selector::new(None, Some("submit".to_string()), Vec::new());

                    assert!(matches_selector(&element_data, &selector));
                }
            }

            describe "if class is specified" {
                describe "element has no class" {
                    #[rstest]
                    fn false_if_element_has_no_class() {
                        let element_data = ElementData::new("button".to_string(), AttributeMap::new());
                        let selector = Selector::new(None, None, Vec::from(["cls".to_string()]));

                        assert!(!matches_selector(&element_data, &selector))
                    }
                }

                describe "element has one or more classes" {
                    #[rstest(element_classes, selector_classes,
                        case("a", Vec::from(["a"])),
                        case("r u s t", Vec::from(["r"])),
                        case("r u s t", Vec::from(["u", "s", "t", "r"])),
                        case("r u s t l a n g u a g e", Vec::from(["u", "s", "t", "r"])),
                    )]
                    fn true_if_all_classes_in_selector_is_specified_in_element(element_classes: &str, selector_classes: Vec<&str>) {
                        let element_data =
                            ElementData::new("button".to_string(), AttributeMap::from([("class".to_string(), element_classes.to_string())]));
                        let selector = Selector::new(None, None, selector_classes.iter().map(|c| c.to_string()).collect());

                        assert!(matches_selector(&element_data, &selector))
                    }

                    #[rstest(element_classes, selector_classes,
                        case("a", Vec::from(["b"])),
                        case("a b c", Vec::from(["a", "b", "c", "d"])),
                    )]
                    fn false_if_any_class_in_selector_is_not_specified_in_element(element_classes: &str, selector_classes: Vec<&str>) {
                        let element_data =
                            ElementData::new("button".to_string(), AttributeMap::from([("class".to_string(), element_classes.to_string())]));
                        let selector = Selector::new(None, None, selector_classes.iter().map(|c| c.to_string()).collect());

                        assert!(!matches_selector(&element_data, &selector))
                    }

                }
            }
        }

        describe "'matching_rules' returns rules matched for the element" {
            #[rstest(element_data, stylesheet_data, expected_rules,
                case(
                    ElementData::new("a".to_string(), AttributeMap::new()),
                    "",
                    Vec::new()
                ),
                case(
                    ElementData::new("a".to_string(), AttributeMap::new()),
                    "a { display: block; }",
                    Vec::from([
                        Rule::new(
                            Vec::from([Selector::new(Some("a".to_string()), None, Vec::new())]),
                            Vec::from([Declaration::new("display".to_string(), Value::String("block".to_string()))])
                        )
                    ])
                ),
                case(
                    ElementData::new("a".to_string(), AttributeMap::new()),
                    "a { display: block; } a { display: flex; }",
                    Vec::from([
                        Rule::new(
                            Vec::from([Selector::new(Some("a".to_string()), None, Vec::new())]),
                            Vec::from([Declaration::new("display".to_string(),Value::String("block".to_string()))])
                        ),
                        Rule::new(
                            Vec::from([Selector::new(Some("a".to_string()), None, Vec::new())]),
                            Vec::from([Declaration::new("display".to_string(), Value::String("flex".to_string()))])
                        )
                    ])
                ),
                case(
                    ElementData::new("a".to_string(), AttributeMap::from([
                        ("id".to_string(), "id".to_string()),
                        ("class".to_string(), "link link1 link2".to_string())
                    ])),
                    "a { display: block; }  b { height: 10px; } a.link { display: flex; } #id { color: red; } a.link1.link2 { background-color: green; }",
                    Vec::from([
                        Rule::new(
                            Vec::from([Selector::new(Some("a".to_string()), None, Vec::new())]),
                            Vec::from([Declaration::new("display".to_string(), Value::String("block".to_string()))])
                        ),
                        Rule::new(
                            Vec::from([Selector::new(Some("a".to_string()), None, Vec::from(["link".to_string()]))]),
                            Vec::from([Declaration::new("display".to_string(), Value::String("flex".to_string()))])
                        ),
                        Rule::new(
                            Vec::from([Selector::new(None, Some("id".to_string()), Vec::new())]),
                            Vec::from([Declaration::new("color".to_string(), Value::String("red".to_string()))])
                        ),
                        Rule::new(
                            Vec::from([Selector::new(Some("a".to_string()), None, Vec::from(["link1".to_string(), "link2".to_string()]))]),
                            Vec::from([Declaration::new("background-color".to_string(), Value::String("green".to_string()))])
                        ),
                    ])
                ),
            )]
            fn matched_rules_for_the_element(element_data: ElementData, stylesheet_data: &str, expected_rules: Vec<Rule>) {
                let stylesheet = css::parse(stylesheet_data.to_string());
                let rules = matching_rules(&element_data, &stylesheet);

                dbg!(&rules);
                assert_eq!(rules.len(), expected_rules.len());

                for ((_, rule), expected_rule) in rules.iter().zip(expected_rules) {
                    assert_eq!(**rule, expected_rule)
                }
            }
        }

        describe "'specified_values' returns a propaty map for the element in specificity order of rules" {
            #[rstest(element_data, stylesheet_data, expected_property_map,
                case(
                    ElementData::new("a".to_string(), AttributeMap::new()),
                    "",
                    PropertyMap::new(),
                ),
                case(
                    ElementData::new("a".to_string(), AttributeMap::new()),
                    "a { display: block; }",
                    PropertyMap::from([
                        ("display".to_string(), Value::String("block".to_string()))
                    ]),
                ),
                case(
                    ElementData::new("a".to_string(), AttributeMap::new()),
                    "a { display: block; } a { display: flex; }",
                    PropertyMap::from([
                        ("display".to_string(), Value::String("flex".to_string()))
                    ])
                ),
                case(
                    ElementData::new("a".to_string(), AttributeMap::from([
                        ("id".to_string(), "id".to_string()),
                        ("class".to_string(), "link link1 link2".to_string())
                    ])),
                    "a { display: block; }  b { height: 10px; } a.link { display: flex; } #id { color: red; color: blue; color: white; color: black; } a.link1.link2 { background-color: green; }",
                    PropertyMap::from([
                        ("display".to_string(), Value::String("flex".to_string())),
                        ("color".to_string(), Value::String("black".to_string())),
                        ("background-color".to_string(), Value::String("green".to_string())),
                    ])
                ),
            )]
            fn matched_property_map_for_the_element_in_specificity_order(element_data: ElementData, stylesheet_data: &str, expected_property_map: PropertyMap) {
                let stylesheet = css::parse(stylesheet_data.to_string());
                assert_eq!(specified_values(&element_data, &stylesheet), expected_property_map);
            }
        }
    }
}
