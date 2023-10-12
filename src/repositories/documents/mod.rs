use crate::models;
use crate::forms;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct Document {
    conn: PgConnection
}

impl Document {
    pub fn new() -> Document {
        Document {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &forms::Document) -> models::Document {
        use crate::schema::documents;
        diesel::insert_into(documents::table)
            .values(doc)
            .returning(models::Document::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<models::Document> {
        use crate::schema::documents::dsl::*;
        documents.select(models::Document::as_select())
             .load(&mut self.conn)
             .expect("Error loading documents")
    }
}
