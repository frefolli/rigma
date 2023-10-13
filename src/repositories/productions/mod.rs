use crate::models;
use crate::forms;
use crate::schema;
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

    pub fn create(&mut self, production: &forms::Production) -> models::Production {
        diesel::insert_into(schema::productions::table)
            .values(production)
            .returning(models::Production::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new production")
    }

    pub fn import(&mut self, productions: &Vec<forms::Production>) -> Vec<models::Production> {
        diesel::insert_into(schema::productions::table)
            .values(productions)
            .returning(models::Production::as_returning())
            .get_results(&mut self.conn)
            .expect("Error saving new production")
    }

    pub fn all(&mut self) -> Vec<models::Production> {
        use crate::schema::productions::dsl::*;
        productions.select(models::Production::as_select())
             .load(&mut self.conn)
             .expect("Error loading productions")
    }

    pub fn delete_all(&mut self) {
        use crate::schema::productions::dsl::*;
        diesel::delete(productions)
            .execute(&mut self.conn)
            .expect("Error deleting all productions");
    }

    pub fn filter(&mut self, asset_id: i32) -> Vec<models::Production> {
        use crate::schema::productions::dsl::*;
        productions.filter(asset.eq(asset_id))
            .select(models::Production::as_returning())
            .load(&mut self.conn)
            .expect("Error finding production")
    }
}
