use serde::Serialize;
use std::fmt;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Production {
    pub left: String,
    pub right: Vec<String>
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rrep = (&self.right).into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "({} => [{}])", self.left, rrep)
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
