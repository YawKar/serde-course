# Exercise 4: Serializing Enums and Nested Structs

In this exercise, you'll use the Serde container attribute `#[serde(deny_unknown_fields)]` to disable tolerance for unknown fields during serialization.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `ContainmentClass` enum and the `Object` struct.
3. Consider test cases in the `tests` module and notice that:
   - `ContainmentClass::Neutralized` must be serialized into `"Neutralized"`
   - `ContainmentClass::Euclid` must be serialized into `"Euclid":{"reason":"<String>"}`
   - `ContainmentClass::Thaumiel` must be serialized into `"Thaumiel":<u32>`
4. Use Serde derive macros to make `ContainmentClass` and `Object` serializable and deserializable.
5. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

