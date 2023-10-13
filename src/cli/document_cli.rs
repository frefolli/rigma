use crate::repositories;
use crate::forms;

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
