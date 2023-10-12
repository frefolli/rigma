use crate::models;
use crate::forms;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct Symbol {
    conn: PgConnection
}

impl Symbol {
    pub fn new() -> Symbol {
        Symbol {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &forms::Symbol) -> models::Symbol {
        use crate::schema::symbols;
        diesel::insert_into(symbols::table)
            .values(doc)
            .returning(models::Symbol::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<models::Symbol> {
        use crate::schema::symbols::dsl::*;
        symbols.select(models::Symbol::as_select())
             .load(&mut self.conn)
             .expect("Error loading symbols")
    }
}
