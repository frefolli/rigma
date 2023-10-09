use rigma::models::*;
use diesel::prelude::*;
use rigma::*;

fn main() {
    use rigma::schema::documents::dsl::*;

    let connection = &mut establish_connection();
    let results = documents
        .select(Document::as_select())
        .load(connection)
        .expect("Error loading documents");

    println!("Displaying {} documents", results.len());
    for document in results {
        println!("{}", document.name);
        println!("-----------\n");
        match document.description {
            Some(desc) => println!("{}", desc),
            None => {}
        }
    }
}
