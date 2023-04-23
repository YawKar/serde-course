# Exercise 23: Serialize Fields Conditionally

In this exercise, you'll use Serde attributes `#[serde(skip_serializing_if = "...")]` to skip field for serialization if specified function returns true, and `#[serde(skip)]` to skip field for both serialization and deserialization. Also we will use `#[serde(default = "...")]` to set default value for skipped field during deserialization.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Address` struct. It's already setup for serializing and deserializing.
3. Notice that, by default, all `Option::None` values are serialized into `null` by Serde.
4. Also check out `default_version` static method of the `Address` struct.
5. What we need:
   - we don't want to serialize `Option::None` values at all (we want to skip them, may be you will find `std::Option::is_none` method useful)
   - we don't want the `version` field to be neither serialized, nor deserialized
   - but in case of deserialization we want the `version` field to be equal to the `Address::default_version`
6. Consider test cases in the `tests` module for better understanding of the serialized representation.
7. Read the description of [skip_serializing_if attribute](https://serde.rs/field-attrs.html#skip_serializing_if).
8. Read the description of [skip attribute](https://serde.rs/field-attrs.html#skip).
9. Read the description of [default with path attribute](https://serde.rs/field-attrs.html#default--path)
10. Try to implement skip_serializing_if attribute on each of `Address` struct's fields!
11. Try to implement skip attribute and default with path attribute on the `version` field of the `Address` struct!
12. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

