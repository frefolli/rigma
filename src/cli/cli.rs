use super::document_cli;
use super::asset_cli;

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
