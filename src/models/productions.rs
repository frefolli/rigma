use serde::Serialize;
use std::fmt;
use diesel::prelude::{Selectable, Queryable};

#[derive(Serialize, Clone, Debug, PartialEq, Selectable, Queryable)]
#[diesel(table_name = crate::schema::productions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Production {
    pub id: i32,
    pub asset: i32,
    pub left: i32,
    pub right: Vec<i32>
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rrep = (&self.right).into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ");
        write!(f, "({} / {} : {} => [{}])", self.id, self.asset, self.left, rrep)
    }
}

impl Production {
    pub fn new() -> Production {
        return Production {
            id: 0,
            asset: 0,
            left: 0,
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
