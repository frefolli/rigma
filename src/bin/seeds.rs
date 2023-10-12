use rigma::repositories::documents::DocumentRepository;

fn main() {
    let mut docs = DocumentRepository::new();
    for doc in docs.all() {
        println!("doc: {}", doc);
    }
}
