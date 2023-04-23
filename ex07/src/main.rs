use std::collections::HashMap;

fn main() {
    println!("To run tests for the ex. 7 use: cargo test");
}

/* Place your
 * code here */
#[cfg_attr(test, derive(Debug, PartialEq))] // Needed for tests, can be replaced by plain derive
enum Message {
    Request {
        id: String,
        method: String,
        params: HashMap<String, String>,
    },
    Response {
        id: String,
        body: String,
    },
}

#[cfg(test)]
mod tests {
    use super::Message;
    use std::collections::HashMap;

    #[test]
    fn serialize_request() {
        let message = Message::Request {
            id: "123".to_owned(),
            method: "POST".to_owned(),
            params: HashMap::from([(String::from("api_key"), String::from("123"))]),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(
            r#"{"id":"123","method":"POST","params":{"api_key":"123"}}"#,
            serialized
        );
    }

    #[test]
    fn serialize_response() {
        let message = Message::Response {
            id: "123".to_owned(),
            body: "Hello world!".to_owned(),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(r#"{"id":"123","body":"Hello world!"}"#, serialized);
    }

    #[test]
    fn serialize_and_deserialize_request() {
        let message = Message::Request {
            id: "123".to_owned(),
            method: "POST".to_owned(),
            params: HashMap::from([(String::from("api_key"), String::from("123"))]),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }

    #[test]
    fn serialize_and_deserialize_response() {
        let message = Message::Response {
            id: "123".to_owned(),
            body: "Hello world!".to_owned(),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(message, deserialized);
    }
}
