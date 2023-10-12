use crate::models::symbols::Symbol as SymbolModel;
use crate::forms::symbols::Symbol as SymbolForm;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct SymbolRepository {
    conn: PgConnection
}

impl SymbolRepository {
    pub fn new() -> SymbolRepository {
        SymbolRepository {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &SymbolForm) -> SymbolModel {
        use crate::schema::symbols;
        diesel::insert_into(symbols::table)
            .values(doc)
            .returning(SymbolModel::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<SymbolModel> {
        use crate::schema::symbols::dsl::*;
        symbols.select(SymbolModel::as_select())
             .load(&mut self.conn)
             .expect("Error loading symbols")
    }
}
