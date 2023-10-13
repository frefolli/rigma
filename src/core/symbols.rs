use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub terminality: bool
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(symbol name: \"{}\" terminality: {})",
            self.name, self.terminality
        )
    }
}

impl Symbol {
    pub fn new() -> Symbol {
        return Symbol {
            name: "".to_string(),
            terminality: false
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
