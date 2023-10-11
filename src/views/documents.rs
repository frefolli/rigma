use serde::Serialize;
use super::assets::{Asset};
use std::fmt::Display;

#[derive(Serialize)]
pub struct Document {
    pub name: String,
    pub description: String,
    pub asset: Asset
}

impl Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write(f, "(document name: \"{}\" description: \"{}\" asset: {})",
            self.name, self.description, self.asset
        )
    }
}

impl Document {
    pub fn new() -> Document {
        return Document {
            name: "".to_string(),
            description: "".to_string(),
            asset: Asset::new()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
