# Exercise 22: Capture Extra Fields Into HashMap Using Flatten

In this exercise, you'll use Serde attribute `#[serde(flatten)]` to capture extra fields from input representation into `HashMap` inside your struct.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `SniffedJSON` struct. It's already setup for serializing and deserializing.
3. Notice the `entries: std::collections::HashMap<String, serde_json::Value>` field. It will capture all extra fields from the input representation.
4. Consider test cases in the `tests` module for better understanding of the serialized representation.
5. Read the description of [flatten attribute](https://serde.rs/field-attrs.html#flatten).
6. Additionally check out the page solely dedicated to [flatten attribute use cases](https://serde.rs/attr-flatten.html).
7. Try to implement flatten attribute for the `entries` field of the `SniffedJSON` struct!
8. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

