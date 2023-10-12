use serde::Serialize;
use std::fmt;
use diesel::prelude::{Selectable, Queryable};

#[derive(Serialize, Clone, Debug, PartialEq, Selectable, Queryable)]
#[diesel(table_name = crate::schema::productions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Branch {
    pub id: i32,
    pub production: i32,
    pub left: i32,
    pub right: Vec<i32>
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(branch id: {} production: {} symbol: {} index: {})", self.id, self.production, self.symbol, self.index)
    }
}

impl Branch {
    pub fn new() -> Branch {
        return Branch {
            id: 0,
            production: 0,
            symbol: 0,
            index: 0
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
