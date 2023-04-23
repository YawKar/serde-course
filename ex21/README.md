# Exercise 21: Flattening Nested Struct in Serialization and Zipping in Deserialization

In this exercise, you'll use Serde attribute `#[serde(flatten)]` to include other struct's fields into the chosen one during serialization and deserialization.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Pagination` struct. It's already setup for serializing and deserializing.
3. Explore the `Cryptocurrency` struct. It's also already setup for serializing and deserializing.
4. Explore the `CryptoEntries` struct. It uses pagination, for example, for querying.
5. Consider test cases in the `tests` module for better understanding of the serialized representation.
6. We need:
   - to accumulate `Cryptocurrency` objects into `entries` vector
   - (during deserialization) to zip `limit`, `offset` and `total` fields of the input object into `pagination: Pagination` instance
   - (during serialization) to flatten `pagination: Pagination` instance's fields into 3 separate fields: `limit`, `offset` and `total`
7. Read the description of [flatten attribute](https://serde.rs/field-attrs.html#flatten)
8. Try to implement flatten attribute for the `pagination` field of the `CryptoEntries` struct!
9. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

