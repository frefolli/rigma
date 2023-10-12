#[cfg(test)]
mod assets {
    use rigma::models::{Asset};

    #[test]
    fn new_empty() {
        let asset = Asset::new();
        assert_eq!(asset.id, 0);
        assert_eq!(asset.document, 0);
    }

    #[test]
    fn new_fill() {
        let mut asset = Asset::new();
        asset.id = 1;
        asset.document = 1;
        assert_eq!(asset.id, 1);
        assert_eq!(asset.document, 1);
    }

    #[test]
    fn new_to_string() {
        let asset = Asset::new();
        let rep = format!("{}", asset);
        assert_eq!(rep, "(asset id: 0 document: 0)");
    }

    #[test]
    fn new_to_json() {
        let asset = Asset::new();
        let rep = format!("{}", asset.to_json());
        assert_eq!(rep, "{\"id\":0,\"document\":0}");
    }
}
