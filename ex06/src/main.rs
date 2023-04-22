fn main() {
    println!("To run tests for the ex. 6 use: cargo test -p ex6");
}

/* Place your
 * code here */
#[cfg_attr(test, derive(Debug, PartialEq))] // Needed for tests, can be replaced by plain derive
enum Element {
    Container(Vec<Element>),
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn serialize_text_element_and_check_by_byte() {
        let element = Element::Text("Hello world".to_string());
        let serialized = serde_json::to_string(&element).unwrap();
        assert_eq!(r#"{"type":"Text","content":"Hello world"}"#, serialized);
    }

    #[test]
    fn serialize_container_element_and_check_by_byte() {
        let element = Element::Container(vec![
            Element::Text(String::from("First Text")),
            Element::Text(String::from("Second Text")),
        ]);
        let serialized = serde_json::to_string(&element).unwrap();
        assert_eq!(
            r#"{"type":"Container","content":[{"type":"Text","content":"First Text"},{"type":"Text","content":"Second Text"}]}"#,
            serialized
        );
    }
}
