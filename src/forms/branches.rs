use serde::Serialize;
use std::fmt;
use diesel::prelude::{Insertable};

#[derive(Serialize, Clone, Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::branches)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Branch {
    pub production: i32,
    pub symbol: i32,
    pub index: i32
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(branch production: {} symbol: {} index: {})", self.production, self.symbol, self.index)
    }
}

impl Branch {
    pub fn new() -> Branch {
        return Branch {
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
