#[cfg(test)]
mod symbols {
    use rigma::forms::{Symbol};

    #[test]
    fn new_empty() {
        let symbol = Symbol::new();
        assert_eq!(symbol.asset, 0);
        assert_eq!(symbol.name, "".to_string());
        assert_eq!(symbol.terminality, false);
    }

    #[test]
    fn new_fill() {
        let mut symbol = Symbol::new();
        symbol.asset = 1;
        symbol.name = "name".to_string();
        symbol.terminality = true;
        assert_eq!(symbol.asset, 1);
        assert_eq!(symbol.name, "name".to_string());
        assert_eq!(symbol.terminality, true);
    }

    #[test]
    fn new_to_string() {
        let symbol = Symbol::new();
        let rep = format!("{}", symbol);
        assert_eq!(rep, "(symbol asset: 0 name: \"\" terminality: false)");
    }

    #[test]
    fn new_to_json() {
        let symbol = Symbol::new();
        let rep = format!("{}", symbol.to_json());
        assert_eq!(rep, "{\"asset\":0,\"name\":\"\",\"terminality\":false}");
    }
}
