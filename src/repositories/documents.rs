use crate::models::documents::{Document};

pub fn create_document(conn: &mut PgConnection, name: &str, description: &str) -> Document {
    use crate::schema::documents;

    let mut new_document = Document::new();
    new_document.name = name.to_string();
    new_document.description = description.to_string();

    diesel::insert_into(documents::table)
        .values(&new_document)
        .returning(Document::as_returning())
        .get_result(conn)
        .expect("Error saving new document")
}
