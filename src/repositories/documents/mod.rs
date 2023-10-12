use crate::models::documents::Document as DocumentModel;
use crate::forms::documents::Document as DocumentForm;
use crate::connection::{get_connection};
use diesel::prelude::*;

pub fn create(doc: &DocumentForm) -> DocumentModel {
    use crate::schema::documents;
    let conn = &mut get_connection();

    diesel::insert_into(documents::table)
        .values(doc)
        .returning(DocumentModel::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn all() -> Vec<DocumentModel> {
    use crate::schema::documents::dsl::*;
    let conn = &mut get_connection();
    documents.select(DocumentModel::as_select())
         .load(conn)
         .expect("Error loading documents")
}
