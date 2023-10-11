use serde::Serialize;
use std::fmt::Disaply;

#[derive(Serialize, Clone)]
pub struct Symbol {
    pub name: String,
    pub terminality: bool
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(symbol name: \"{}\" terminality: {}",
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
