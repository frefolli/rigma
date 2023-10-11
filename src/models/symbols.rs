use serde::Serialize;
use std::fmt;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Symbol {
    pub asset: i32,
    pub name: String,
    pub terminality: bool
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(symbol asset: {} name: \"{}\" terminality: {})",
            self.asset, self.name, self.terminality
        )
    }
}

impl Symbol {
    pub fn new() -> Symbol {
        return Symbol {
            asset: 0,
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
