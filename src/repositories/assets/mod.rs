use crate::models;
use crate::forms;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct Asset {
    conn: PgConnection
}

impl Asset {
    pub fn new() -> Asset {
        Asset {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &forms::Asset) -> models::Asset {
        use crate::schema::assets;
        diesel::insert_into(assets::table)
            .values(doc)
            .returning(models::Asset::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<models::Asset> {
        use crate::schema::assets::dsl::*;
        assets.select(models::Asset::as_select())
             .load(&mut self.conn)
             .expect("Error loading assets")
    }
}
