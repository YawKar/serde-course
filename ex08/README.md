# Exercise 8: Adding Trait Bound for Serialization/Deserialization

In this exercise, you'll use the Serde container attribute `#[serde(bound = "...")]` to enable trait bounds for serialization and deserialization.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the templated `Response` enum. Notice that it has templated body field.
3. Explore the `UserInfo` struct and `NonSensitiveData` marker trait.
4. Notice that tests pass by default. After you make your changes, they should pass as well.
5. Add `impl NonSensitiveData` for `UserInfo` struct.
6. Read the description of [bound attribute](https://serde.rs/container-attrs.html#bound)
7. Try to implement trait bound that forbids serialization/deserialization of types that don't impl `NonSensitiveData` in /* Place your bound macro here */ section!
8. You may face error saying that `T` doesn't implement `serde::Serialize` and/or `serde::Deserialize`.
   To solve it, populate your trait bound with `T: serde::Serialize + for<'a> serde::Deserialize<'a>`.
   For now, you should not really bother about the lifetime specification `for<'a>`.
   We basically tell Serde that our `T` type is serializable and deserializable using Serde traits.
9. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

