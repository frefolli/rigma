use diesel::prelude::*;
use crate::schema::documents;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = documents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Document {
    pub id: i32,
    pub name: String,
    pub description: String
}

impl Document {
    pub fn new() -> Document {
        return Document {
            id: 0,
            name: "".to_string(),
            description: "".to_string()
        }
    }
}
