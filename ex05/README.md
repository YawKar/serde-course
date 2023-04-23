# Exercise 5: Serializing Internally Tagged Enum

In this exercise, you'll use the Serde container attribute `#[serde(tag = "...")]` to enable "internally tagged" representation of enum.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Message` enum.
3. Consider test cases in the `tests` module and notice that:
   - serialization should contain `message_type` field with value representing a variant of the enum
   - serde will automatically serialize and deserialize the `params: HashMap<String, String>` field of the `Request` variant
4. Read the description of [tag attribute](https://serde.rs/container-attrs.html#tag)
5. Additionally check out [different representations of enums in serde](https://serde.rs/enum-representations.html)
6. Try to implement internally tagged representation in /* Place your code here */ section!
7. Don't forget to derive `serde::Serialization` and `serde::Deserialization`.
8. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

