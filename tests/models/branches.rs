#[cfg(test)]
mod branches {
    use rigma::models::{Branch};

    #[test]
    fn new_empty() {
        let branch = Branch::new();
        assert_eq!(branch.id, 0);
        assert_eq!(branch.production, 0);
        assert_eq!(branch.symbol, 0);
        assert_eq!(branch.index, 0);
    }

    #[test]
    fn new_fill() {
        let mut branch = Branch::new();
        branch.id = 1;
        branch.production = 1;
        branch.symbol = 1;
        branch.index = 1;
        assert_eq!(branch.id, 1);
        assert_eq!(branch.production, 1);
        assert_eq!(branch.symbol, 1);
        assert_eq!(branch.index, 1);
    }

    #[test]
    fn new_to_string() {
        let branch = Branch::new();
        let rep = format!("{}", branch);
        assert_eq!(rep, "(branch id: 0 production: 0 symbol: 0 index: 0)");
    }

    #[test]
    fn new_to_json() {
        let branch = Branch::new();
        let rep = format!("{}", branch.to_json());
        assert_eq!(rep, "{\"id\":0,\"production\":0,\"symbol\":0,\"index\":0}");
    }
}
