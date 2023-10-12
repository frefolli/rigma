use rigma::repositories;

fn main() {
    println!("Document::Dump");
    for document in repositories::Document::new().all() {
        println!("{}", document);
    }

    println!("Asset::Dump");
    for asset in repositories::Asset::new().all() {
        println!("{}", asset);
    }
    
    println!("Symbol::Dump");
    for symbol in repositories::Symbol::new().all() {
        println!("{}", symbol);
    }
    
    println!("Production::Dump");
    for production in repositories::Production::new().all() {
        println!("{}", production);
    }
    
    println!("Branch::Dump");
    for branch in repositories::Branch::new().all() {
        println!("{}", branch);
    }
}
