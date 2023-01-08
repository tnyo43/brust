use crate::dom::{AttributeMap, Node};
use crate::parser::Parser;

struct HTMLParser {
    base: Parser,
}

impl HTMLParser {
    fn new(input: String) -> Self {
        HTMLParser {
            base: Parser::new(input),
        }
    }

    fn parse_tag_string(&mut self) -> String {
        self.base.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> Node {
        self.base.consume_whitespace();
        match self.base.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> Node {
        dbg!("parse");
        Node::text(self.base.consume_while(|c| c != '<'))
    }

    fn parse_attribute(&mut self) -> (String, String) {
        let name = self.parse_tag_string();
        assert!(self.base.consume_char() == '=');
        let open_quote = self.base.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.parse_tag_string();
        let close_quote = self.base.consume_char();
        assert!(close_quote == open_quote);
        (name, value)
    }

    fn parse_attributes(&mut self) -> AttributeMap {
        let mut attributes = AttributeMap::new();

        loop {
            self.base.consume_whitespace();

            if self.base.next_char() == '>' {
                break;
            }

            let (name, value) = self.parse_attribute();
            attributes.insert(name, value);
        }

        attributes
    }

    fn parse_element(&mut self) -> Node {
        assert!(self.base.consume_char() == '<');

        let name = self.parse_tag_string();
        let attributes = self.parse_attributes();

        assert!(self.base.consume_char() == '>');

        let children = self.parse_elements();

        assert!(self
            .base
            .start_with(format!("</{name}>").to_string().as_str()));
        loop {
            if self.base.consume_char() == '>' {
                break;
            }
        }

        Node::element(name, attributes, children)
    }

    fn parse_elements(&mut self) -> Vec<Node> {
        let mut elements = Vec::<Node>::new();
        loop {
            self.base.consume_whitespace();

            assert!(!self.base.eof());
            if self.base.start_with("</") {
                break;
            }

            elements.push(self.parse_node());
        }

        elements
    }
}

pub fn parse(data: String) -> Node {
    let mut parser = HTMLParser::new(data.to_string());
    parser.parse_node()
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use rstest::*;
    use speculate::speculate;

    use super::*;

    speculate! {
        describe "'parse_element'" {
            describe "returns element without any attribute and children" {
                #[rstest()]
                fn test_parse_element_with_simple_element() {
                    let mut html_parser = HTMLParser::new("<div></div>".to_string());

                    assert_eq!(html_parser.parse_element(), Node::element("div".to_string(), AttributeMap::new(), vec![]));
                }

                #[should_panic]
                #[rstest]
                fn test_parse_should_panic_element_without_closing_tag() {
                    let mut html_parser = HTMLParser::new("<input>".to_string());

                    html_parser.parse_element();
                }

                #[should_panic]
                #[rstest]
                fn test_parse_should_panic_element_with_invalid_tag() {
                    let mut html_parser = HTMLParser::new("<div />".to_string());

                    html_parser.parse_element();
                }
            }

            describe "returns element with attribute" {
                #[rstest(input, expected,
                    case(
                        "<div id=\"container\"></div>",
                        Node::element("div".to_string(), AttributeMap::from([("id".to_string(), "container".to_string())]), Vec::<Node>::new())
                    ),
                    case(
                        "<p id=\"paragraph1\" class='ppp'></p>",
                        Node::element("p".to_string(), AttributeMap::from([("id".to_string(), "paragraph1".to_string()), ("class".to_string(), "ppp".to_string())]), Vec::<Node>::new())
                    )
                )]
                fn test_parse_attributes_with_single_attribute(input: &str, expected: Node) {
                    let mut html_parser = HTMLParser::new(input.to_string());

                    assert_eq!(html_parser.parse_element(), expected)
                }
            }

            describe "returns element with children" {
                #[rstest(input, expected,
                    case(
                        "<div>hello</div>",
                        Node::element("div".to_string(), AttributeMap::new(), Vec::<Node>::from([
                            Node::text("hello".to_string())
                        ]))
                    ),
                    case(
                        "<div><p>hello</p><button>click</button></div>",
                        Node::element("div".to_string(), AttributeMap::new(), Vec::<Node>::from([
                            Node::element("p".to_string(), AttributeMap::new(), Vec::from([
                                Node::text("hello".to_string())
                            ])),
                            Node::element("button".to_string(), AttributeMap::new(), Vec::from([
                                Node::text("click".to_string())
                            ]))
                        ]))
                    ),
                    case(
                        "<div><div><div><div></div></div></div></div>",
                        Node::element("div".to_string(), AttributeMap::new(), Vec::<Node>::from([
                            Node::element("div".to_string(), AttributeMap::new(), Vec::<Node>::from([
                                Node::element("div".to_string(), AttributeMap::new(), Vec::<Node>::from([
                                    Node::element("div".to_string(), AttributeMap::new(), Vec::<Node>::new())
                                ]))
                            ]))
                        ]))
                    )
                )]
                fn test_parse_element_with_children(input: &str, expected: Node) {
                    let mut html_parser = HTMLParser::new(input.to_string());

                    assert_eq!(html_parser.parse_element(), expected)
                }
            }
        }

        describe "'parse' returns DOM nodes" {
            #[rstest(input, expected,
                case(
                    "<div></div>",
                    Node::element("div".to_string(), AttributeMap::new(), Vec::new())
                ),
                case(
                    "hello world!",
                    Node::text("hello world!".to_string())
                ),
                case(
                    "    <div id='main'   class=\"container\">   <h1>   title</h1>hi<button   onclick=\"function\">   click me!</button>    <p>   abc<b>   def</b>ghi</p>    </div>    ",
                    Node::element("div".to_string(), AttributeMap::from([("id".to_string(), "main".to_string()), ("class".to_string(), "container".to_string())]), Vec::from([
                        Node::element("h1".to_string(), AttributeMap::new(), Vec::from([
                            Node::text("title".to_string())
                        ])),
                        Node::text("hi".to_string()),
                        Node::element("button".to_string(), AttributeMap::from([("onclick".to_string(), "function".to_string())]), Vec::from([
                            Node::text("click me!".to_string())
                        ])),
                        Node::element("p".to_string(), AttributeMap::new(), Vec::from([
                            Node::text("abc".to_string()),
                            Node::element("b".to_string(), AttributeMap::new(), Vec::from([
                                Node::text("def".to_string()),
                            ])),
                            Node::text("ghi".to_string()),
                        ])),
                    ]))
                )
            )]
            fn test_parse_valid_html(input: &str, expected: Node) {
                assert_eq!(parse(input.to_string()), expected);
            }

            #[should_panic]
            #[rstest(input,
                case("<div></div"),
                case("<div></p>"),
                case("<div id=class></div>"),
            )]
            fn test_should_panic_parse_invalid_html(input: &str) {
                parse(input.to_string());
            }
        }
    }
}
