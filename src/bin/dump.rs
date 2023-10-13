mod asset_cli {
    use rigma::repositories;
    use rigma::forms;

    pub fn execute(args: &Vec<String>) {
        if args.len() > 0 {
            let cmd = &args[0];
            if cmd == "list"{
                list();
            } else if cmd == "create" {
                create(&args[1]);
            } else if cmd == "find" {
                find(&args[1]);
            } else {
                unknown_command(cmd);
            }
        }
    }

    fn list() {
        for asset in repositories::Asset::new().all() {
            println!("{}", asset);
        }
    }

    fn create(document_id: &str) {
        println!("/asset => create({})", document_id);
        let asset = repositories::Asset::new().create(&forms::Asset::from(document_id.parse().unwrap()));
        println!("Created: {}", asset);
    }

    fn find(id: &str) {
        println!("/asset => find({})", id);
        let doc = repositories::Asset::new().find(id.parse().unwrap());
        println!("Found: {}", doc);
    }

    fn unknown_command(cmd: &str) {
        panic!("unknown command {}, expected [list | create | find]", cmd);
    }
}

mod document_cli {
    use rigma::repositories;
    use rigma::forms;

    pub fn execute(args: &Vec<String>) {
        if args.len() > 0 {
            let cmd = &args[0];
            if cmd == "list" {
                list();
            } else if cmd == "create" {
                create(&args[1], &args[2]);
            } else if cmd == "find" {
                find(&args[1]);
            } else {
                unknown_command(cmd);
            }
        }
    }

    fn list() {
        for document in repositories::Document::new().all() {
            println!("{}", document);
        }
    }

    fn create(name: &str, desc: &str) {
        println!("/document => create({}, {})", name, desc);
        let doc = repositories::Document::new().create(&forms::Document::from(name, desc));
        println!("Created: {}", doc);
    }

    fn find(id: &str) {
        println!("/document => find({})", id);
        let doc = repositories::Document::new().find(id.parse().unwrap());
        println!("Found: {}", doc);
    }

    fn unknown_command(cmd: &str) {
        panic!("unknown command {}, expected [list | create | find]", cmd);
    }
}

mod cli {
    use crate::document_cli;
    use crate::asset_cli;
    pub fn execute(args: &Vec<String>) {
        infoline(&args[0]);
        if args.len() > 1 {
            let cmd = &args[1];
            if cmd == "document" {
                document_cli::execute(&args[2..].to_vec());
            } else if cmd == "asset" {
                asset_cli::execute(&args[2..].to_vec());
            } else {
                unknown_command(cmd);
            }
        }
    }

     fn infoline(exec: &str) {
        println!("{} TODO", exec);
     }

    fn unknown_command(cmd: &str) {
        panic!("unknown command {}, expected [document | asset]", cmd);
    }
}

fn main() {
    let args : Vec<String> = std::env::args().into_iter().map(|x| x).collect();
    cli::execute(&args);
}
