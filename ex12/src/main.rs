fn main() {
    println!("To run tests for the ex. 12 use: cargo test");
}

// Imagine that this is an external crate, not a module
mod other_crate {
    // You cannot change anything here because it is not your code.
    // However, there is a workaround to serialize and deserialize such
    // 'remote' types.
    // Good news: all fields are public) So serde can match them to your medium type)
    pub struct TestResult {
        pub mark: char,
        pub percentile: f32,
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(remote = "other_crate::TestResult")]
struct TestResultDef {
    mark: char,
    percentile: f32,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SomeStruct {
    #[serde(with = "TestResultDef")]
    external_result: other_crate::TestResult,
}

#[cfg(test)]
mod tests {
    use super::{other_crate::TestResult, SomeStruct};

    #[test]
    fn serialize_external_test_result() {
        let test_result = SomeStruct {
            // Serializing external struct!
            external_result: TestResult {
                mark: 'A',
                percentile: 0.99,
            },
        };
        let serialized = serde_json::to_string(&test_result).unwrap();
        assert_eq!(
            r#"{"external_result":{"mark":"A","percentile":0.99}}"#,
            serialized
        );
    }

    #[test]
    fn serialize_and_deserialize_external_test_result() {
        let test_result = SomeStruct {
            // Serializing external struct!
            external_result: TestResult {
                mark: 'A',
                percentile: 0.99,
            },
        };
        let serialized = serde_json::to_string(&test_result).unwrap();
        let deserialized: SomeStruct = serde_json::from_str(&serialized).unwrap();
        assert_eq!(test_result.external_result.mark, deserialized.external_result.mark);
        assert_eq!(test_result.external_result.percentile, deserialized.external_result.percentile);
    }
}
