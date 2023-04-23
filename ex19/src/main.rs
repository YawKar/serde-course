fn main() {
    println!("To run tests for the ex. 19 use: cargo test");
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
#[cfg_attr(test, derive(Debug, PartialEq))]
enum Event {
    Opening,
    Closing,
    /* Place your serde attribute here, so any unknown "type" in internally tagged enum will
     * produce Unknown */
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::Event;

    #[test]
    fn deserialize_opening_should_return_opening() {
        let json = r#"{"type":"Opening"}"#;
        let deserialized: Event = serde_json::from_str(json).unwrap();
        assert_eq!(Event::Opening, deserialized);
    }

    #[test]
    fn deserialize_closing_should_return_closing() {
        let json = r#"{"type":"Closing"}"#;
        let deserialized: Event = serde_json::from_str(json).unwrap();
        assert_eq!(Event::Closing, deserialized);
    }

    #[test]
    fn deserialize_unknown_should_return_unknown() {
        let json = r#"{"type":"Unknown"}"#;
        let deserialized: Event = serde_json::from_str(json).unwrap();
        assert_eq!(Event::Unknown, deserialized);
    }

    #[test]
    fn deserialize_any_else_type_should_return_unknown() {
        let json = r#"{"type":"Anything"}"#;
        let deserialized: Event = serde_json::from_str(json).unwrap();
        assert_eq!(Event::Unknown, deserialized);
    }
}
