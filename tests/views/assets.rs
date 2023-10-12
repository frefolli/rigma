#[cfg(test)]
mod assets {
    use rigma::views::{Asset, Symbol, Production};

    #[test]
    fn new_empty() {
        let asset = Asset::new();
        assert_eq!(asset.symbols, Vec::<Symbol>::new());
        assert_eq!(asset.grammar, Vec::<Production>::new());
    }

    #[test]
    fn new_fill() {
        let mut asset = Asset::new();
        let sym = Symbol::new();
        let prod = Production::new();
        asset.symbols.push(sym.clone());
        asset.grammar.push(prod.clone());
        assert_eq!(asset.symbols, Vec::<Symbol>::from([sym]));
        assert_eq!(asset.grammar, Vec::<Production>::from([prod]));
    }

    #[test]
    fn new_to_string() {
        let asset = Asset::new();
        let rep = format!("{}", asset);
        assert_eq!(rep, "(asset symbols: [] grammar: [])");
    }

    #[test]
    fn new_to_json() {
        let asset = Asset::new();
        let rep = format!("{}", asset.to_json());
        assert_eq!(rep, "{\"symbols\":[],\"grammar\":[]}");
    }
}
