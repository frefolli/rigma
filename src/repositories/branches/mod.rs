use crate::models::branches::Branch as BranchModel;
use crate::forms::branches::Branch as BranchForm;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct BranchRepository {
    conn: PgConnection
}

impl BranchRepository {
    pub fn new() -> BranchRepository {
        BranchRepository {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &BranchForm) -> BranchModel {
        use crate::schema::branches;
        diesel::insert_into(branches::table)
            .values(doc)
            .returning(BranchModel::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<BranchModel> {
        use crate::schema::branches::dsl::*;
        branches.select(BranchModel::as_select())
             .load(&mut self.conn)
             .expect("Error loading branches")
    }
}
