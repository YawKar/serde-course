fn main() {
    println!("To run tests for the ex. 13 use: cargo test");
}

// Imagine that this is an external crate, not a module
mod other_crate {
    // You cannot change anything here because it is not your code.
    // However, there is a workaround to serialize and deserialize such
    // 'remote' types.
    // Bad news: all fields are private
    // Good news: there are some getters that serde can use
    pub struct TestResult {
        mark: char,
        percentile: f32,
    }

    impl TestResult {
        // We can use this method to deserialize other_crate::TestResult from our medium type
        pub fn new(mark: char, percentile: f32) -> TestResult {
            TestResult { mark, percentile }
        }
        // We can use this getter for serialization through medium type
        pub fn mark(&self) -> char {
            self.mark
        }
        // As well as we can use this getter for the same serialization
        pub fn percentile(&self) -> f32 {
            self.percentile
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(remote = "other_crate::TestResult")]
struct TestResultDef {
    #[serde(getter = "other_crate::TestResult::mark")]
    mark: char,
    #[serde(getter = "other_crate::TestResult::percentile")]
    percentile: f32,
}

impl From<TestResultDef> for other_crate::TestResult {
    fn from(value: TestResultDef) -> other_crate::TestResult {
        other_crate::TestResult::new(value.mark, value.percentile)
    }
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
    fn serialize_external_result() {
        let some_struct = SomeStruct {
            // Serializing external struct with private fields, yet public getters for them!
            external_result: TestResult::new('A', 0.87),
        };
        let serialized = serde_json::to_string(&some_struct).unwrap();
        assert_eq!(
            r#"{"external_result":{"mark":"A","percentile":0.87}}"#,
            serialized
        );
    }

    #[test]
    fn serialize_and_deserialize_external_result() {
        let some_struct = SomeStruct {
            // Serializing external struct with private fields, yet public getters for them!
            external_result: TestResult::new('A', 0.87),
        };
        let serialized = serde_json::to_string(&some_struct).unwrap();
        let deserialized: SomeStruct = serde_json::from_str(&serialized).unwrap();
        assert_eq!(some_struct.external_result.mark(), deserialized.external_result.mark());
        assert_eq!(some_struct.external_result.percentile(), deserialized.external_result.percentile());
    }
}
