# Exercise 7: Serializing Untagged Enum

In this exercise, you'll use the Serde container attribute `#[serde(untagged)]` to enable "untagged" representation of enum.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Message` enum.
3. Consider test cases in the `tests` module and notice that:
   - serialization does not contain any "type" field whatsoever
   - nested will also have the same "untagged" representation
4. Read the description of [untagged attribute](https://serde.rs/container-attrs.html#untagged)
5. Additionally check out [different representations of enums in serde](https://serde.rs/enum-representations.html)
6. Try to implement adjacently tagged representation in /* Place your code here */ section!
7. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

