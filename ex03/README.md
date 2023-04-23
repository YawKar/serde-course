# Exercise 3: Disabling Tolerance for Unknown Fields

In this exercise, you'll use the Serde container attribute `#[serde(deny_unknown_fields)]` to disable tolerance for unknown fields during serialization.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `MobilePlan` struct.
3. Consider test cases in the `tests` module and notice that:
   - `json` has one extra field, namely, `is_special_offer`
4. Use Serde derive macro to make `MobilePlan` deserializable.
5. Read the description of [deny_unknown_fields attribute](https://serde.rs/container-attrs.html#deny_unknown_fields)
6. Try to implement it in `/* Place your code here */` section!
7. Once you've implemented deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

