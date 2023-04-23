# Exercise 9: Using `Default::default` for Deserialization

In this exercise, you'll use the Serde container attribute `#[serde(default)]` to fallback to default values for fields that are missing in the input.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Point` struct.
3. Consider test cases in the `tests` module and notice that:
   - deserialization from an empty json should work and fallback missing fields' values to that of `Default::default` instance
   - deserialization fallbacks only for missing fields
4. Read the description of [default attribute](https://serde.rs/container-attrs.html#default)
5. Try to implement default attribute in /* Place your serde macro here */ section!
6. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

