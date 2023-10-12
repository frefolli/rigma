use crate::models::documents::Document as DocumentModel;
use crate::forms::documents::Document as DocumentForm;
use diesel_connection::{pg::get_connection, PoolError};

pub fn create(doc: &DocumentForm) -> Document {
    use crate::schema::documents;
    let conn = get_connection().expect("Error connecting to server");

    diesel::insert_into(documents::table)
        .values(doc)
        .returning(Document::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn all() -> Vec<Document> {
    use crate::schema::documents::dsl::documents;
    let conn = get_connection().expect("Error connecting to server");
    posts.select(Post::as_select())
         .load(connection)
         .expect("Error loading posts")
}
