# Exercise 18: Skip Enum Variants for Serialization and/or Deserialization

In this exercise, you'll use Serde attributes `#[serde(skip)]`, `#[serde(skip_serializing)]` and `#[serde(skip_deserializing)]` to selectively forbid serialization and deserialization for an enum variant.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Token` enum. It's already setup for serializing and deserializing.
3. We need Serde to:
   1. fail in case of an attempt to serialize/deserialize `Token::Spacebar` variant
   2. fail in case of an attempt to serialize `Token::Character` variant
   3. fail in case of an attempt to deserialize `Token::Block` variant
4. Consider test cases in the `tests` module for better understanding of desired behaviour.
5. Read the description of [skip attribute](https://serde.rs/variant-attrs.html#skip)
6. Read the description of [skip_serializing attribute](https://serde.rs/variant-attrs.html#skip_serializing)
7. Read the description of [skip_deserializing attribute](https://serde.rs/variant-attrs.html#skip_deserializing)
8. Try to implement these attributes according to the aforementioned desired behaviour!
9. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

