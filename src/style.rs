#[derive(Debug, PartialEq)]
pub struct Declaration {
    name: String,
    value: String,
}

impl Declaration {
    pub fn new(name: String, value: String) -> Self {
        Declaration {
            name: name,
            value: value,
        }
    }
}
