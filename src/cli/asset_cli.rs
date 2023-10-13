use crate::repositories;
use crate::forms;

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
