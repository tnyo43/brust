use crate::{
    parser::Parser,
    style::{Color, Declaration, Rule, Selector, StyleSheet, Unit, Value},
};

struct CSSParser {
    base: Parser,
}

fn parse_value(value: String) -> Value {
    if value.starts_with('#') {
        assert!(value.len() == 7);
        let r = u8::from_str_radix(&value[1..=2], 16).unwrap();
        let g = u8::from_str_radix(&value[3..=4], 16).unwrap();
        let b = u8::from_str_radix(&value[5..=6], 16).unwrap();
        return Value::color(r, g, b);
    }

    if ('0'..='9').contains(&value.chars().next().unwrap()) {
        let (num, unit) = if value.ends_with("px") {
            ((value[..value.len() - 2]).parse::<f32>().unwrap(), Unit::Px)
        } else if value.ends_with("%") {
            (
                (value[..value.len() - 1]).parse::<f32>().unwrap(),
                Unit::Percent,
            )
        } else if value.ends_with("rem") {
            (
                (value[..value.len() - 3]).parse::<f32>().unwrap(),
                Unit::Rem,
            )
        } else if value.ends_with("em") {
            ((value[..value.len() - 2]).parse::<f32>().unwrap(), Unit::Em)
        } else {
            ((value).parse::<f32>().unwrap(), Unit::None)
        };

        return Value::size(num, unit);
    }

    Value::string(value)
}

impl CSSParser {
    fn new(input: String) -> Self {
        CSSParser {
            base: Parser::new(input),
        }
    }

    fn is_valid_identifier_initial_char(&self) -> bool {
        match self.base.next_char() {
            'a'..='z' | 'A'..='Z' => true,
            _ => false,
        }
    }

    fn parse_identifier(&mut self) -> String {
        self.base.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
            _ => false,
        })
    }

    fn parse_selector(&mut self) -> Selector {
        let mut selector = Selector::new(None, None, Vec::new());

        while !self.base.eof() {
            self.base.consume_whitespace();
            match self.base.next_char() {
                '#' => {
                    self.base.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.base.consume_char();
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

        while !self.base.eof() {
            self.base.consume_whitespace();

            selectors.push(self.parse_selector());

            self.base.consume_whitespace();
            if self.base.eof() || self.base.next_char() != ',' {
                break;
            }
            self.base.consume_char();
        }

        selectors
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert!(self.base.consume_char() == '{');

        let mut declarations = Vec::new();

        loop {
            self.base.consume_whitespace();

            if self.base.next_char() == '}' {
                self.base.consume_char();
                break;
            }

            let name = self.parse_identifier();

            self.base.consume_whitespace();
            assert!(self.base.consume_char() == ':');
            self.base.consume_whitespace();

            let valueText = self.base.consume_while(|c| c != ';');
            assert!(self.base.consume_char() == ';');

            declarations.push(Declaration::new(name, parse_value(valueText)));
        }

        declarations
    }

    fn parse_rule(&mut self) -> Rule {
        self.base.consume_whitespace();
        let selectors = self.parse_selectors();

        self.base.consume_whitespace();
        let declarations = self.parse_declarations();

        Rule::new(selectors, declarations)
    }

    fn parse_stylesheet(&mut self) -> StyleSheet {
        let mut rules = Vec::new();

        loop {
            self.base.consume_whitespace();

            if self.base.eof() {
                break;
            }

            rules.push(self.parse_rule());
        }

        StyleSheet::new(rules)
    }
}

pub fn parse(data: String) -> StyleSheet {
    let mut parser = CSSParser::new(data);
    parser.parse_stylesheet()
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    extern crate speculate;

    use rstest::*;
    use speculate::speculate;

    use super::*;

    speculate! {
        describe "'parse_value'" {
            describe "if value start with '#', value is parsed to color" {
                #[rstest(input, expected,
                    case("#000000", Value::color(0, 0, 0)),
                    case("#123456", Value::color(18, 52, 86)),
                    case("#abcdef", Value::color(171, 205, 239)),
                )]
                fn parse_color_code(input: &str, expected: Value) {
                    assert_eq!(parse_value(input.to_string()), expected);
                }

                #[should_panic]
                #[rstest(input,
                    case("#123"),
                    case("#1111111"),
                    case("#zyxwvut"),
                )]
                fn fail_to_parse_with_invalid_color(input: &str) {
                    parse_value(input.to_string());
                }
            }

            describe "if value start with number, value is parsed to size" {
                #[rstest(input, expected,
                    case("10px", Value::size(10.0, Unit::Px)),
                    case("43%", Value::size(43.0, Unit::Percent)),
                    case("1.4em", Value::size(1.4, Unit::Em)),
                    case("0.1rem", Value::size(0.1, Unit::Rem)),
                    case("10000", Value::size(10000.0, Unit::None)),
                )]
                fn parse_color_code(input: &str, expected: Value) {
                    assert_eq!(parse_value(input.to_string()), expected);
                }

                #[should_panic]
                #[rstest(input,
                    case("1hogehogepx"),
                    case("1ab"),
                )]
                fn fail_to_parse_with_invalid_size(input: &str) {
                    parse_value(input.to_string());
                }
            }
        }

        describe "'parse_selectors' parse selector" {
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

        describe "'parse_declarations' parses declaration block" {
            #[rstest]
            fn test_empty_block() {
                let mut css_parser = CSSParser::new("{}".to_string());

                assert_eq!(css_parser.parse_declarations(), Vec::new());
            }

            #[rstest(input, expected,
                case("{}", Vec::new()),
                case("{ display: block; }", Vec::from([Declaration::new("display".to_string(), Value::String("block".to_string()))])),
                case(
                    "{ border: 1px solid #123456; background-color: red; }",
                    Vec::from([Declaration::new("border".to_string(), Value::String("1px solid #123456".to_string())), Declaration::new("background-color".to_string(), Value::String("red".to_string()))])
                )
            )]
            fn test_parse_declarations(input: &str, expected: Vec<Declaration>) {
                let mut css_parser = CSSParser::new(input.to_string());

                assert_eq!(css_parser.parse_declarations(), expected);
            }
        }

        describe "'parse_rule' returns rule" {
            #[rstest(input, expected,
                case(
                    "a#link, b.thin { display: flex; margin-top: 16px; }",
                    Rule::new(
                        Vec::from([
                            Selector::new(Some("a".to_string()), Some("link".to_string()), Vec::new()),
                            Selector::new(Some("b".to_string()), None, Vec::from(["thin".to_string()]))
                        ]),
                        Vec::from([
                            Declaration::new("display".to_string(), Value::String("flex".to_string())),
                            Declaration::new("margin-top".to_string(), Value::String("16px".to_string())),
                        ])
                    )
                ),
            )]
            fn test_rule(input: &str, expected: Rule) {
                let mut css_parser = CSSParser::new(input.to_string());

                assert_eq!(css_parser.parse_rule(), expected);
            }
        }

        describe "'parse' returns stylesheet" {
            #[rstest(data, expected,
                case(
                    "a#link {\n display: flex; color: #d3a003; \n} \n\n  \n .cls, #modal { position: absolute; \n top: 50%; } \n ",
                    StyleSheet::new(Vec::from([
                        Rule::new(
                            Vec::from([Selector::new(Some("a".to_string()), Some("link".to_string()), Vec::new())]),
                            Vec::from([
                                Declaration::new("display".to_string(), Value::String("flex".to_string())),
                                Declaration::new("color".to_string(), Value::String("#d3a003".to_string()))
                            ])
                        ),
                        Rule::new(
                            Vec::from([Selector::new(None, None, Vec::from(["cls".to_string()])), Selector::new(None, Some("modal".to_string()), Vec::new())]),
                            Vec::from([
                                Declaration::new("position".to_string(), Value::String("absolute".to_string())),
                                Declaration::new("top".to_string(), Value::String("50%".to_string())),
                            ])
                        )
                    ]))
                )
            )]
            fn test_parse(data: &str, expected: StyleSheet) {
                assert_eq!(parse(data.to_string()), expected);
            }
        }
    }
}
