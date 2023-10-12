use rigma::repositories;
use rigma::forms;

fn main() {
    let mut repo = repositories::Document::new();
    repo.create(&forms::Document::from("grammar.yml", ""));
    repo.create(&forms::Document::from("clr.yml", ""));
    repo.create(&forms::Document::from("wrong.yml", ""));
    repo.create(&forms::Document::from("bash.yml", ""));
}
