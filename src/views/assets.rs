use serde::Serialize;
use super::symbols::{Symbol};
use super::productions::{Production};
use crate::helpers::strings::{vec_to_string};
use std::fmt::Display;

#[derive(Serialize)]
pub struct Asset {
    pub symbols: Vec<Symbol>,
    pub grammar: Vec<Production>
}

impl Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(asset symbols: {} grammar: {})",
                self.symbols,
                self.grammar)
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
        format!("{}", f);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
