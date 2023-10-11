#[cfg(test)]
mod productions {
    use rigma::views::productions::{Production};

    #[test]
    fn new_empty() {
        let production = Production::new();
        assert_eq!(production.left, "".to_string());
        assert_eq!(production.right, Vec::<String>::new());
    }

    #[test]
    fn new_fill() {
        let mut production = Production::new();
        production.left = "left".to_string();
        production.right = Vec::<String>::from([String::from("ok")]);
        assert_eq!(production.left, "left".to_string());
        assert_eq!(production.right, Vec::<String>::from([String::from("ok")]));
    }

    #[test]
    fn new_to_string() {
        let production = Production::new();
        let rep = format!("{}", production);
        assert_eq!(rep, "( => [])");
    }

    #[test]
    fn new_to_json() {
        let production = Production::new();
        let rep = format!("{}", production.to_json());
        assert_eq!(rep, "{\"left\":\"\",\"right\":[]}");
    }
}
