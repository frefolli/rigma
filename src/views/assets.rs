use serde::Serialize;
use super::symbols::{Symbol};
use super::productions::{Production};
use std::fmt;

#[derive(Serialize, Debug, PartialEq)]
pub struct Asset {
    pub symbols: Vec<Symbol>,
    pub grammar: Vec<Production>
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let srep = (&self.symbols).into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ");
        let grep = (&self.grammar).into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "(asset symbols: [{}] grammar: [{}])", srep, grep)
    }
}

impl Asset {
    pub fn new() -> Asset {
        return Asset {
            symbols: [].to_vec(),
            grammar: [].to_vec()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
