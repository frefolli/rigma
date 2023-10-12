use crate::models::productions::Production as ProductionModel;
use crate::forms::productions::Production as ProductionForm;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct ProductionRepository {
    conn: PgConnection
}

impl ProductionRepository {
    pub fn new() -> ProductionRepository {
        ProductionRepository {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &ProductionForm) -> ProductionModel {
        use crate::schema::productions;
        diesel::insert_into(productions::table)
            .values(doc)
            .returning(ProductionModel::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<ProductionModel> {
        use crate::schema::productions::dsl::*;
        productions.select(ProductionModel::as_select())
             .load(&mut self.conn)
             .expect("Error loading productions")
    }
}
