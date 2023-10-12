use crate::models::documents::Document as DocumentModel;
use crate::forms::documents::Document as DocumentForm;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct DocumentRepository {
    conn: PgConnection
}

impl DocumentRepository {
    pub fn new() -> DocumentRepository {
        DocumentRepository {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &DocumentForm) -> DocumentModel {
        use crate::schema::documents;
        diesel::insert_into(documents::table)
            .values(doc)
            .returning(DocumentModel::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<DocumentModel> {
        use crate::schema::documents::dsl::*;
        documents.select(DocumentModel::as_select())
             .load(&mut self.conn)
             .expect("Error loading documents")
    }
}
