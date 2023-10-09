use diesel::prelude::*;
use crate::schema::symbols;

#[derive(Queryable, Selectable)]
#[diesel(table_name = symbols)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Symbol {
    pub id: i32,
    pub document: i32,
    pub name: String,
    pub terminality: bool
}

#[derive(Insertable)]
#[diesel(table_name = symbols)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSymbol {
    pub document: i32,
    pub name: String,
    pub terminality: bool
}
