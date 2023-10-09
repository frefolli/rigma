use rigma::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut description = String::new();

    println!("What would you like your name to be?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        name, EOF
    );
    stdin().read_to_string(&mut description).unwrap();

    let document = create_document(connection, name, Some(&description));
    println!("\nSaved draft {} with id {}", name, document.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
