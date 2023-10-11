use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize, Clone)]
pub struct Production {
    pub left: String,
    pub right: Vec<String>
}

impl Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} => {})",
            self.left, self.right
        )
    }
}

impl Production {
    pub fn new() -> Production {
        return Production {
            left: "".to_string(),
            right: [].to_vec()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
