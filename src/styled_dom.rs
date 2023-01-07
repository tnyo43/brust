use crate::dom::ElementData;
use crate::style::Selector;

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

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use rstest::*;
    use speculate::speculate;

    use super::*;
    use crate::dom::AttributeMap;

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
    }
}
