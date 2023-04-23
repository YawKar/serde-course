# Exercise 19: Deserializing Unknown Enum Variant Into Default Chosen

In this exercise, you'll use Serde attribute `#[serde(other)]` to fallback to chosen variant when input contains unknown enum variant type.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Event` enum. It's already setup for serializing and deserializing.
3. Notice that `Event` enum has `#[serde(tag = "type")]` attribute. It means that it has [internally tagged representation](https://serde.rs/enum-representations.html#internally-tagged).
4. Consider test cases in the `tests` module for better understanding of desired behaviour.
5. Read the description of [other attribute](https://serde.rs/variant-attrs.html#other)
6. Try to implement other attribute for the `Event::Unknown` variant!
7. Once you've implemented deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

