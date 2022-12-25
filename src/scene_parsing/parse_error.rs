use std::fmt::{Display, Debug};


pub struct ParseError {
    pub msg: String,
    pub json: serde_json::Value
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.msg, self.json)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.msg, self.json)
    }
}

