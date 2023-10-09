pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewDocument, Document};

pub fn create_document(conn: &mut PgConnection, name: &str, description: Option<&str>) -> Document {
    use crate::schema::documents;

    let new_document = NewDocument { name, description };

    diesel::insert_into(documents::table)
        .values(&new_document)
        .returning(Document::as_returning())
        .get_result(conn)
        .expect("Error saving new document")
}

