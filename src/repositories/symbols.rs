use crate::models::symbols::{Symbol};

pub fn create_symbol(conn: &mut PgConnection, document: &i32, name: &str, terminality: &bool) -> Symbol {
    use crate::schema::symbols;

    let mut new_symbol = Symbol::new();
    new_symbol.document = document;
    new_symbol.name = name.to_string();
    new_symbol.description = description.to_string();

    diesel::insert_into(symbols::table)
        .values(&new_symbol)
        .returning(Symbol::as_returning())
        .get_result(conn)
        .expect("Error saving new symbol")
}
