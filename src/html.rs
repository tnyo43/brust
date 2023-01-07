use crate::dom::{AttributeMap, Node};

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn new(input: String) -> Self {
        Parser {
            pos: 0,
            input: input,
        }
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn start_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, current_char) = iter.next().unwrap();
        self.pos += 1;
        current_char
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && condition(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn parse_tag_string(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> Node {
        self.consume_whitespace();
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> Node {
        Node::text(self.consume_while(|c| c != '>'))
    }

    fn parse_attribute(&mut self) -> (String, String) {
        let name = self.parse_tag_string();
        assert!(self.consume_char() == '=');
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.parse_tag_string();
        let close_quote = self.consume_char();
        assert!(close_quote == open_quote);
        (name, value)
    }

    fn parse_attributes(&mut self) -> AttributeMap {
        let mut attributes = AttributeMap::new();

        loop {
            self.consume_whitespace();

            if self.next_char() == '>' {
                break;
            }

            let (name, value) = self.parse_attribute();
            attributes.insert(name, value);
        }

        attributes
    }

    fn parse_element(&mut self) -> Node {
        assert!(self.consume_char() == '<');

        let name = self.parse_tag_string();
        let attributes = self.parse_attributes();

        assert!(self.consume_char() == '>');

        let children = Vec::<Node>::new();

        assert!(self.start_with(format!("</{name}>").to_string().as_str()));

        loop {
            if self.consume_char() == '>' {
                break;
            }
        }

        Node::element(name, attributes, children)
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
        describe "'next_char' returns n-th char of input" {
            #[rstest(pos, expected,
                case(0, 'h'),
                case(1, 'e'),
                case(10, ' '),
            )]
            fn test_next_char(pos: usize, expected: char) {
                let parser = Parser {
                    pos: pos,
                    input: "hello rust world!".to_string()
                };
                assert_eq!(parser.next_char(), expected);
            }
        }

        describe "'start_with' judges if the substring of the input from the position start with a string" {
            #[rstest(pos, text, expected,
                case(0, "hell", true),
                case(4, "hell", false),
                case(11, "w", true),
                case(11, "world", true),
            )]
            fn test_start_with(pos: usize, text: &str, expected: bool) {
                let parser = Parser {
                    pos: pos,
                    input: "hello rust world!".to_string()
                };
                assert_eq!(parser.start_with(text), expected);
            }
        }

        describe "'eof' judges if the position is over the end of file of the input" {
            #[rstest(input, pos, expected,
                case("hello", 4, false),
                case("hello", 5, true),
                case("", 0, true),
                case("aaa", 1000, true),
            )]
            fn test_eof(input: &str, pos: usize, expected: bool) {
                let parser = Parser {
                    pos: pos,
                    input: input.to_string()
                };
                assert_eq!(parser.eof(), expected);
            }
        }

        describe "'consume_while'" {
            describe "returns string while find a first character does not sutisfy the condition" {
                #[rstest(input, pos, condition, expected,
                    case("hello world!", 0, |c| c != ' ', "hello"),
                    case("hello world!", 3, |c| c != 'l', ""),
                    case("hello world!", 7, |c: char| c.is_alphanumeric(), "orld"),
                )]
                fn test_consume_while_with_condition<F>(input: &str, pos: usize, condition: F, expected: &str)
                where
                    F: Fn(char) -> bool
                {
                    let mut parser = Parser {
                        pos: pos,
                        input: input.to_string()
                    };
                    assert_eq!(parser.consume_while(condition), expected);
                }
            }

            describe "returns whole string if all the characters sutisfy the condition" {
                #[rstest(input, pos, condition, expected,
                    case("hello world!", 0, |_| true, "hello world!"),
                    case("hello world!", 6, |c: char| c.is_alphabetic() || c == '!', "world!"),
                )]
                fn test_consume_while_until_the_end<F>(input: &str, pos: usize, condition: F, expected: &str)
                where
                    F: Fn(char) -> bool
                {
                    let mut parser = Parser {
                        pos: pos,
                        input: input.to_string()
                    };
                    assert_eq!(parser.consume_while(condition), expected);
                }
            }
        }

        describe "'consume_whitespace' ignores a sequence of whitespace" {
            #[rstest]
            fn test_consume_whitespace() {
                 let mut parser = Parser {
                    pos: 0,
                    input: "    a    b c".to_string()
                };
                parser.consume_whitespace();
                assert_eq!(parser.next_char(), 'a');
            }
        }

        describe "'parse_element'" {
            describe "returns element without any attribute and children" {
                #[rstest()]
                fn test_parse_element_with_simple_element() {
                    let mut parser = Parser::new("<div></div>".to_string());

                    assert_eq!(parser.parse_element(), Node::element("div".to_string(), AttributeMap::new(), vec![]));
                }

                #[should_panic]
                #[rstest]
                fn test_parse_should_panic_element_without_closing_tag() {
                    let mut parser = Parser::new("<input>".to_string());

                    parser.parse_element();
                }

                #[should_panic]
                #[rstest]
                fn test_parse_should_panic_element_with_invalid_tag() {
                    let mut parser = Parser::new("<div />".to_string());

                    parser.parse_element();
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
                    let mut parser = Parser::new(input.to_string());

                    assert_eq!(parser.parse_element(), expected)
                }
            }
        }
    }
}
