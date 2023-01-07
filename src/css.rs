use crate::{
    parser::Parser,
    style::{Declaration, Selector},
};

struct CSSParser {
    parser: Parser,
}

impl CSSParser {
    fn new(input: String) -> Self {
        CSSParser {
            parser: Parser::new(input),
        }
    }

    fn is_valid_identifier_initial_char(&self) -> bool {
        match self.parser.next_char() {
            'a'..='z' | 'A'..='Z' => true,
            _ => false,
        }
    }

    fn parse_identifier(&mut self) -> String {
        self.parser.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
            _ => false,
        })
    }

    fn parse_selector(&mut self) -> Selector {
        let mut selector = Selector::new(None, None, Vec::new());

        while !self.parser.eof() {
            self.parser.consume_whitespace();
            match self.parser.next_char() {
                '#' => {
                    self.parser.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.parser.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                _ if self.is_valid_identifier_initial_char() => {
                    selector.tag = Some(self.parse_identifier());
                }
                _ => {
                    break;
                }
            }
        }

        selector
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        while !self.parser.eof() {
            self.parser.consume_whitespace();

            selectors.push(self.parse_selector());

            self.parser.consume_whitespace();
            if self.parser.eof() || self.parser.next_char() != ',' {
                break;
            }
            self.parser.consume_char();
        }

        selectors
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert!(self.parser.consume_char() == '{');

        let mut declarations = Vec::new();

        loop {
            self.parser.consume_whitespace();

            if self.parser.next_char() == '}' {
                self.parser.consume_char();
                break;
            }

            let name = self.parse_identifier();

            self.parser.consume_whitespace();
            assert!(self.parser.consume_char() == ':');
            self.parser.consume_whitespace();

            let value = self.parser.consume_while(|c| c != ';');
            assert!(self.parser.consume_char() == ';');

            declarations.push(Declaration::new(name, value));
        }

        declarations
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
        describe "'parse_selectors' parse selector" {
            #[ignore]
            #[rstest(input, expected,
                case(
                    ".hoge__fizz-bar",
                    Vec::from([Selector::new(None, None, Vec::from(["hoge__fizz-bar".to_string()]))])
                ),
                case(
                    "div.a.b.c.d",
                    Vec::from([Selector::new(Some("div".to_string()), None,  Vec::from(["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string() ]))]),
                ),
                case(
                    "button#submit_name.main",
                    Vec::from([Selector::new(Some("button".to_string()), Some("submit_name".to_string()), Vec::from(["main".to_string() ]))])
                ),
                case(
                    "h1,h2,    h3",
                    Vec::from([
                        Selector::new(Some("h1".to_string()), None, Vec::new()),
                        Selector::new(Some("h2".to_string()), None, Vec::new()),
                        Selector::new(Some("h3".to_string()), None, Vec::new()),
                    ])
                ),
                case(
                    "#xxx,h2.hoge,#bar.hugahuga",
                    Vec::from([
                        Selector::new(None, Some("xxx".to_string()), Vec::new()),
                        Selector::new(Some("h2".to_string()), None, Vec::from(["hoge".to_string()])),
                        Selector::new(None, Some("bar".to_string()), Vec::from(["hugahuga".to_string()])),
                    ])
                ),
            )]
            fn test_parse_tag_id_class(input: &str, expected: Vec::<Selector>) {
                let mut css_parser = CSSParser::new(input.to_string());

                assert_eq!(css_parser.parse_selectors(), expected);
            }
        }

        describe "'parse_declarations' parse declaration block" {
            #[rstest]
            fn test_empty_block() {
                let mut css_parser = CSSParser::new("{}".to_string());

                assert_eq!(css_parser.parse_declarations(), Vec::new());
            }

            #[rstest(input, expected,
                case("{}", Vec::new()),
                case("{ display: block; }", Vec::from([Declaration::new("display".to_string(), "block".to_string())])),
                case(
                    "{ border: 1px solid #123456; background-color: red; }",
                    Vec::from([Declaration::new("border".to_string(), "1px solid #123456".to_string()), Declaration::new("background-color".to_string(), "red".to_string())])
                )
            )]
            fn test_parse_declarations(input: &str, expected: Vec<Declaration>) {
                let mut css_parser = CSSParser::new(input.to_string());

                assert_eq!(css_parser.parse_declarations(), expected);
            }
        }
    }
}
