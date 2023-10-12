#[cfg(test)]
mod productions {
    use rigma::models::productions::{Production};

    #[test]
    fn new_empty() {
        let production = Production::new();
        assert_eq!(production.id, 0);
        assert_eq!(production.asset, 0);
        assert_eq!(production.left, 0);
    }

    #[test]
    fn new_fill() {
        let mut production = Production::new();
        production.id = 1;
        production.asset = 1;
        production.left = 1;
        assert_eq!(production.id, 1);
        assert_eq!(production.asset, 1);
        assert_eq!(production.left, 1);
    }

    #[test]
    fn new_to_string() {
        let production = Production::new();
        let rep = format!("{}", production);
        assert_eq!(rep, "(production id: 0 asset: 0 left: 0)");
    }

    #[test]
    fn new_to_json() {
        let production = Production::new();
        let rep = format!("{}", production.to_json());
        assert_eq!(rep, "{\"id\":0,\"asset\":0,\"left\":0}");
    }
}
