# Exercise 17: Deserializing Enum Variants with Different Aliases

In this exercise, you'll use Serde attribute `#[serde(alias = "...")]` to deserialize enum variants from different names.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Color` enum. It's already setup for serializing and deserializing.
3. We need Serde to:
   1. be able to deserialize `Color::Red` not only from "Red" literal but also from "Chili red" and "Rust"
   2. be able to deserialize `Color::Blue` not only from "Blue" literal but also from "Azure" and "Aero"
   3. be able to deserialize `Color::Green` not only from "Green" literal but also from "Lime" and "Harlequin"
4. Consider test cases in the `tests` module for better understanding of desired behaviour.
5. Read the description of [alias attribute](https://serde.rs/variant-attrs.html#alias)
6. Try to implement alias attribue for each variant of the `Color` enum!
7. Once you've implemented deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

