use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Production {
    pub left: String,
    pub right: Vec<String>
}

impl Production {
    pub fn new() -> Production {
        return Production {
            left: "".to_string(),
            right: [].to_vec()
        }
    }

    pub fn to_string(&self) -> String {
        return "".to_string();
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
