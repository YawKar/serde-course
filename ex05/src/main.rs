use std::collections::HashMap;

fn main() {
    println!("To run tests for the ex. 5 use: cargo test");
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
    fn serialize_and_check_request() {
        let message = Message::Request {
            id: String::from("1322-sdf3-adsf-5555"),
            method: String::from("POST"),
            params: HashMap::from([
                (String::from("api_key"), String::from("123456789")),
                (
                    String::from("Referer"),
                    String::from("https://stackoverflow.com/"),
                ),
            ]),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        assert!(serialized.contains(r#""message_type":"Request""#));
        assert_eq!(message, serde_json::from_str(&serialized).unwrap());
    }

    #[test]
    fn serialize_and_check_response() {
        let message = Message::Response {
            id: String::from("1322-sdf3-adsf-5555"),
            body: String::from("Hello world"),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        assert!(serialized.contains(r#""message_type":"Response""#));
        assert_eq!(message, serde_json::from_str(&serialized).unwrap());
    }
}
