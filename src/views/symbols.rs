use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Symbol {
    pub name: String,
    pub terminality: bool
}

impl Symbol {
    pub fn new() -> Symbol {
        return Symbol {
            name: "".to_string(),
            terminality: false
        }
    }

    pub fn to_string(&self) -> String {
        return "".to_string();
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
