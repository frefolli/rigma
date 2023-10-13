#[cfg(test)]
mod productions {
    use rigma::forms::{Production};

    #[test]
    fn new_empty() {
        let production = Production::new();
        assert_eq!(production.asset, 0);
        assert_eq!(production.left, 0);
    }

    #[test]
    fn new_fill() {
        let production = Production::from(1, 1);
        assert_eq!(production.asset, 1);
        assert_eq!(production.left, 1);
    }

    #[test]
    fn new_to_string() {
        let production = Production::new();
        let rep = format!("{}", production);
        assert_eq!(rep, "(production asset: 0 left: 0)");
    }

    #[test]
    fn new_to_json() {
        let production = Production::new();
        let rep = format!("{}", production.to_json());
        assert_eq!(rep, "{\"asset\":0,\"left\":0}");
    }
}
