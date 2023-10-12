use rigma::repositories;

fn main() {
    repositories::Branch::new().delete_all();
    repositories::Production::new().delete_all();
    repositories::Symbol::new().delete_all();
    repositories::Asset::new().delete_all();
    repositories::Document::new().delete_all();
}
