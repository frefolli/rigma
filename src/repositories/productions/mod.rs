use crate::models;
use crate::forms;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct Production {
    conn: PgConnection
}

impl Production {
    pub fn new() -> Production {
        Production {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &forms::Production) -> models::Production {
        use crate::schema::productions;
        diesel::insert_into(productions::table)
            .values(doc)
            .returning(models::Production::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<models::Production> {
        use crate::schema::productions::dsl::*;
        productions.select(models::Production::as_select())
             .load(&mut self.conn)
             .expect("Error loading productions")
    }
}
