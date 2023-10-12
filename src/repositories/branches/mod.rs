use crate::models;
use crate::forms;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct Branch {
    conn: PgConnection
}

impl Branch {
    pub fn new() -> Branch {
        Branch {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &forms::Branch) -> models::Branch {
        use crate::schema::branches;
        diesel::insert_into(branches::table)
            .values(doc)
            .returning(models::Branch::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<models::Branch> {
        use crate::schema::branches::dsl::*;
        branches.select(models::Branch::as_select())
             .load(&mut self.conn)
             .expect("Error loading branches")
    }
}
