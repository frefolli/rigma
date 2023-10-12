#[cfg(test)]
mod documents {
    use rigma::views::{Document};

    #[test]
    fn new_empty() {
        let doc = Document::new();
        assert_eq!(doc.name, "".to_string());
        assert_eq!(doc.description, "".to_string());
    }

    #[test]
    fn new_fill() {
        let mut doc = Document::new();
        doc.name = "name".to_string();
        doc.description = "desc".to_string();
        assert_eq!(doc.name, "name".to_string());
        assert_eq!(doc.description, "desc".to_string());
    }

    #[test]
    fn new_to_string() {
        let doc = Document::new();
        let rep = format!("{}", doc);
        assert_eq!(rep, "(document name: \"\" description: \"\" asset: (asset symbols: [] grammar: []))");
    }

    #[test]
    fn new_to_json() {
        let doc = Document::new();
        let rep = format!("{}", doc.to_json());
        assert_eq!(rep, "{\"name\":\"\",\"description\":\"\",\"asset\":{\"symbols\":[],\"grammar\":[]}}");
    }
}
