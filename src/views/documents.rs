use serde::Serialize;
use super::assets::{Asset};

#[derive(Serialize)]
pub struct Document {
    pub name: String,
    pub description: String,
    pub asset: Asset
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
        let mut rep = "(document".to_string();
        if self.name.len() > 0 {
            rep.push_str(format!(" name: \"{}\"",
                                 self.name).as_str());
        }
        if self.description.len() > 0 {
            rep.push_str(format!(" description: \"{}\"",
                                 self.description).as_str());
        }
        rep.push_str(format!(" asset: {}", self.asset.to_string()).as_str());
        return rep + ")";
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
