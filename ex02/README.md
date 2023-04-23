# Exercise 2: Changing Cases of Fields'

In this exercise, you'll use the Serde container attribute `#[serde(rename_all = "...")]` to change case of fields' names.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Person` and `Cryptocurrency` structs.
3. Consider test cases in the `tests` module and notice that:
   - `Person` struct's fields should be serialized into `camelCase`
   - `Person` struct's fields should be deserialized from `SCREAMING-KEBAB-CASE`
   - `Cryptocurrency` struct's fields should be both serialized and deserialized with `PascalCase`
4. Use Serde derive macros to make both structs serializable and deserializable (refer to the solution for the first exercise).
5. Read the description of [rename_all attribute and its independent cases](https://serde.rs/container-attrs.html#rename_all)
6. Try to implement it in `/* Place your code here */` sections!
7. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

