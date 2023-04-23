# Exercise 6: Serializing Adjacently Tagged Enum

In this exercise, you'll use the Serde container attribute `#[serde(tag = "...", content = "...")]` to enable "adjacently tagged" representation of enum.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Element` enum.
3. Consider test cases in the `tests` module and notice that:
   - serialization should contain `type` field with value representing an enum variant
   - serialization should contain `content` field with value representing the actual content of an enum variant
   - nested will also have the same "adjacently tagged" representation
4. Read the description of [tag and content attributes](https://serde.rs/container-attrs.html#tag--content)
5. Additionally check out [different representations of enums in serde](https://serde.rs/enum-representations.html)
6. Try to implement adjacently tagged representation in /* Place your code here */ section!
7. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

