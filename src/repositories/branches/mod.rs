use crate::models;
use crate::forms;
use crate::schema;
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

    pub fn create(&mut self, branch: &forms::Branch) -> models::Branch {
        diesel::insert_into(schema::branches::table)
            .values(branch)
            .returning(models::Branch::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn import(&mut self, branches: &Vec<forms::Branch>) -> Vec<models::Branch> {
        diesel::insert_into(schema::branches::table)
            .values(branches)
            .returning(models::Branch::as_returning())
            .get_results(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<models::Branch> {
        use crate::schema::branches::dsl::*;
        branches.select(models::Branch::as_select())
             .load(&mut self.conn)
             .expect("Error loading branches")
    }

    pub fn delete_all(&mut self) {
        use crate::schema::branches::dsl::*;
        diesel::delete(branches)
            .execute(&mut self.conn)
            .expect("Error deleting all branches");
    }
}
