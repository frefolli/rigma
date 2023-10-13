use rigma::cli;

fn main() {
    let args : Vec<String> = std::env::args().into_iter().map(|x| x).collect();
    cli::execute(&args);
}
