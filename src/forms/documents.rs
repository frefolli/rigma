use serde::Serialize;
use std::fmt;
use diesel::prelude::{Insertable};

#[derive(Serialize, Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::documents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Document {
    pub name: String,
    pub description: String,
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(document name: \"{}\" description: \"{}\")",
            self.name, self.description
        )
    }
}

impl Document {
    pub fn new() -> Document {
        return Document {
            name: "".to_string(),
            description: "".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
