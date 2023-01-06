pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn start_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
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
    }
}
