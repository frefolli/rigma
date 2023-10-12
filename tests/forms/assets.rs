#[cfg(test)]
mod assets {
    use rigma::forms::{Asset};

    #[test]
    fn new_empty() {
        let asset = Asset::new();
        assert_eq!(asset.document, 0);
    }

    #[test]
    fn new_fill() {
        let mut asset = Asset::from(1);
        assert_eq!(asset.document, 1);
    }

    #[test]
    fn new_to_string() {
        let asset = Asset::new();
        let rep = format!("{}", asset);
        assert_eq!(rep, "(asset document: 0)");
    }

    #[test]
    fn new_to_json() {
        let asset = Asset::new();
        let rep = format!("{}", asset.to_json());
        assert_eq!(rep, "{\"document\":0}");
    }
}
