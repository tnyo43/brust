use crate::{parser::Parser, style::Declaration};

struct CSSParser {
    parser: Parser,
}

impl CSSParser {
    fn new(input: String) -> Self {
        CSSParser {
            parser: Parser::new(input),
        }
    }

    fn parse_name(&mut self) -> String {
        self.parser.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' => true,
            _ => false,
        })
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

            let name = self.parse_name();

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
