#[cfg(test)]
mod productions {
    use rigma::core::{Production};

    #[test]
    fn new_empty() {
        let production = Production::new();
        assert_eq!(production.left, 0);
        assert_eq!(production.right, Vec::<i32>::new());
    }

    #[test]
    fn new_fill() {
        let mut production = Production::new();
        production.left = 1;
        production.right = Vec::<>::from([0]);
        assert_eq!(production.left, 1);
        assert_eq!(production.right, Vec::<i32>::from([0]));
    }

    #[test]
    fn new_to_string() {
        let production = Production::new();
        let rep = format!("{}", production);
        assert_eq!(rep, "(0 => [])");
    }

    #[test]
    fn new_to_json() {
        let production = Production::new();
        let rep = format!("{}", production.to_json());
        assert_eq!(rep, "{\"left\":0,\"right\":[]}");
    }
}
