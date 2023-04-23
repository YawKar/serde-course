fn main() {
    println!("To run tests for the ex. 23 use: cargo test");
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    district: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    building: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip)]
    #[serde(default = "Address::default_version")]
    version: String,
}

impl Address {
    fn default_version() -> String {
        "1.0.0".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Address;

    #[test]
    fn serialize_to_empty_json_if_all_none() {
        let address = Address {
            city: None,
            district: None,
            post_index: None,
            street: None,
            building: None,
            state: None,
            version: String::new(),
        };
        let serialized = serde_json::to_string(&address).unwrap();
        assert_eq!("{}", serialized);
    }

    #[test]
    fn serialize_if_some() {
        let address = Address {
            city: Some("New York".to_string()),
            district: Some("Manhattan".to_owned()),
            post_index: Some("10118".to_owned()),
            street: Some("33rd Street".to_owned()),
            building: Some("Empire State Building".to_owned()),
            state: Some("New York".to_owned()),
            version: String::new(),
        };
        let serialized = serde_json::to_string(&address).unwrap();
        assert!(!serialized.contains("version"), "'version' field should not be serialized!");
        assert_eq!(
            "{\
                \"city\":\"New York\",\
                \"district\":\"Manhattan\",\
                \"post_index\":\"10118\",\
                \"street\":\"33rd Street\",\
                \"building\":\"Empire State Building\",\
                \"state\":\"New York\"\
            }",
            serialized
        );
    }

    #[test]
    fn deserialize_version_from_default() {
        let json = serde_json::json!(
            {
                "city": "New York",
                "district": "Manhattan",
                "post_index": "10118",
                "street": "33rd Street",
                "building": "Empire State Building",
                "state": "New York",
            }
        ).to_string();
        let deserialized: Address = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.version, Address::default_version());
    }
}
