use serde::Serialize;
use super::symbols::{Symbol};
use super::productions::{Production};

#[derive(Serialize)]
pub struct Asset {
    pub symbols: Vec<Symbol>,
    pub grammar: Vec<Production>
}

impl Asset {
    pub fn new() -> Asset {
        return Asset {
            symbols: [].to_vec(),
            grammar: [].to_vec()
        }
    }

    pub fn to_string(&self) -> String {
        return "".to_string();
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
