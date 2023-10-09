use diesel::prelude::*;
use crate::schema::documents;

#[derive(Queryable, Selectable)]
#[diesel(table_name = documents)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Document {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Insertable)]
#[diesel(table_name = documents)]
pub struct NewDocument<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}
