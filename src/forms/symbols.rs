use serde::Serialize;
use std::fmt;
use diesel::prelude::{Insertable};

#[derive(Serialize, Clone, Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::symbols)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Symbol {
    pub asset: i32,
    pub name: String,
    pub terminality: bool
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(symbol asset: {} name: \"{}\" terminality: {})",
            self.asset, self.name, self.terminality
        )
    }
}

impl Symbol {
    pub fn new() -> Symbol {
        Symbol::from(0, "", false)
    }

    pub fn from(asset: i32, name: &str, terminality: bool) -> Symbol {
        return Symbol {
            asset: asset,
            name: name.to_string(),
            terminality: terminality
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
