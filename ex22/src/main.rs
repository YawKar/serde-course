fn main() {
    println!("To run tests for the ex. 22 use: cargo test");
}

#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
struct SniffedJSON {
    key: String,
    /* Place here serde attribute, so any extra fields will be captured by this map */
    entries: std::collections::HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::SniffedJSON;

    #[test]
    fn deserialize_json_without_key_field_should_fail() {
        let json = serde_json::json!(
            {
                "some_unknown_key": 100,
                "some_other_key": {
                    "some_inner_key": 123,
                }
            }
        )
        .to_string();
        serde_json::from_str::<SniffedJSON>(&json)
            .expect_err("Deserializing SniffedJSON without 'key' field should be illegal!");
    }

    #[test]
    fn deserialize_json_only_with_key_field_should_work() {
        let json = r#"{"key":"123456789"}"#;
        serde_json::from_str::<SniffedJSON>(json)
            .expect("Should be able to deserialize json with only 'key' field.");
    }

    #[test]
    fn deserialize_json_with_extra_keys_should_work() {
        let json = serde_json::json! (
            {
                "key": "123456789",
                "some_unknown_key": 100,
                "some_other_key": {
                    "some_inner_key": 123,
                }
            }
        )
        .to_string();
        let deserialized = serde_json::from_str::<SniffedJSON>(&json)
            .expect("Should be deserialized because 'key' field is in place.");
        assert_eq!(deserialized.key, "123456789");
        assert_eq!(
            deserialized.entries.len(),
            2,
            "Number of keys in entries should equal 2."
        );
        assert!(deserialized.entries.contains_key("some_unknown_key"));
        assert!(deserialized
            .entries
            .get("some_other_key")
            .expect("entries should have some_other_key key")
            .as_object()
            .expect("some_other_key should contain object")
            .get("some_inner_key")
            .expect("some_other_key object must contain some_inner_key key")
            .as_u64()
            .expect("some_other_key must be parsable as u64")
            .eq(&123));
    }
}
