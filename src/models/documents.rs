use serde::Serialize;
use std::fmt;
use diesel::prelude::{Selectable, Queryable};

#[derive(Serialize, Debug, PartialEq, Selectable, Queryable)]
#[diesel(table_name = crate::schema::documents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Document {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(document id: {} name: \"{}\" description: \"{}\")",
            self.id, self.name, self.description
        )
    }
}

impl Document {
    pub fn new() -> Document {
        return Document {
            id: 0,
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
