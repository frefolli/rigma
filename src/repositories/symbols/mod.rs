use crate::models;
use crate::forms;
use crate::schema;
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

    pub fn create(&mut self, sym: &forms::Symbol) -> models::Symbol {
        diesel::insert_into(schema::symbols::table)
            .values(sym)
            .returning(models::Symbol::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new symbol")
    }

    pub fn import(&mut self, syms: &Vec<forms::Symbol>) -> Vec<models::Symbol> {
        diesel::insert_into(schema::symbols::table)
            .values(syms)
            .returning(models::Symbol::as_returning())
            .get_results(&mut self.conn)
            .expect("Error saving new symbol")
    }

    pub fn all(&mut self) -> Vec<models::Symbol> {
        use crate::schema::symbols::dsl::*;
        symbols.select(models::Symbol::as_select())
             .load(&mut self.conn)
             .expect("Error loading symbols")
    }

    pub fn delete_all(&mut self) {
        use crate::schema::symbols::dsl::*;
        diesel::delete(symbols)
            .execute(&mut self.conn)
            .expect("Error deleting all symbols");
    }
}
