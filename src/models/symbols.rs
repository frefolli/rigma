use serde::Serialize;
use std::fmt;
use diesel::prelude::{Selectable, Queryable};

#[derive(Serialize, Clone, Debug, PartialEq, Selectable, Queryable)]
#[diesel(table_name = crate::schema::symbols)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Symbol {
    pub id: i32,
    pub asset: i32,
    pub name: String,
    pub terminality: bool
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(symbol id: {} asset: {} name: \"{}\" terminality: {})",
            self.id, self.asset, self.name, self.terminality
        )
    }
}

impl Symbol {
    pub fn new() -> Symbol {
        return Symbol {
            id: 0,
            asset: 0,
            name: "".to_string(),
            terminality: false
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
