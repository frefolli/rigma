use serde::Serialize;
use std::fmt;
use diesel::prelude::{Insertable};

#[derive(Serialize, Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::assets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Asset {
    pub document: i32
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(asset document: {})", self.document)
    }
}

impl Asset {
    pub fn new() -> Asset {
        return Asset {
            document: 0
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
