# Exercise 20: Deserializing Missing Fields with `Default::default` or Our Own Function

In this exercise, you'll use Serde attributes `#[serde(default)]` and `#[serde(default = "...")]` to fallback to chosen defaults for missing fields in input.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `User` struct. It's already setup for serializing and deserializing.
3. Notice that `User` has static method `default_description` that returns default user profile description.
4. We need:
   - to keep `nickname` and `password_hash` fields required
   - to fallback to `Default::default` value for the `status: bool` field in case this field is missing
   - to fallback to `User::default_description` value for the `short_description: String` field in case this field is missing
5. Consider test cases in the `tests` module for better understanding of desired behaviour.
6. Read the description of [default attribute](https://serde.rs/field-attrs.html#default)
7. Read the description of [default with path attribute](https://serde.rs/field-attrs.html#default--path)
8. Try to implement these attributes for `status` and `short_description` fields respectively!
9. Once you've implemented deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

