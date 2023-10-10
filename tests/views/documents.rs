#[cfg(test)]
mod documents {
    use rigma::views::documents::{Document};

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
        let str = doc.to_string();
        assert_eq!(str, "(document asset: )");
    }

    #[test]
    fn new_to_json() {
        let doc = Document::new();
        let str = doc.to_json();
        assert_eq!(str, "{\"name\":\"\",\"description\":\"\",\"asset\":{\"symbols\":[],\"grammar\":[]}}");
    }
}
