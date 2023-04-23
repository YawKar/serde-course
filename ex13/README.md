# Exercise 13: Serializing Remote Types from External Crates via Getters

In this exercise, you'll use Serde attributes `#[serde(remote = "...")]`, `#[serde(with = "...")]` and `#[serde(getter = "...")]` to use intermediate 'definition' type for serializing and deserializing remote types from external crates. This time remote type's fields are private and we only have getters for them. Luckily, Serde has a workaround for this little inconvenience.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `other_crate` module, pretend that this is somebody else's crate, therefore, you cannot change any code inside it.
3. Explore the `SomeStruct` struct that contains field `external_result` of type `other_crate::TestResult`.
4. We want to be able to serialize and deserialize `SomeStruct`, but in order to do that we need somehow derive `serde::Serialize` and `serde::Deserialize` for the `other_crate::TestResult` type. (Rust's orphan rules forbids to implement external traits on external types)
5. Consider test cases in the `tests` module and notice that:
   - `other_crate::TestResult` should seamlessly serialize and deserialize
6. Read the description of [remote attribute](https://serde.rs/container-attrs.html#remote)
7. Additionally read the full page dedicated to [remote serde attribute](https://serde.rs/remote-derive.html)
8. Try to declare appropriate `TestResultDef` definition type and then link it to the remote type `other_crate::TestResult` using remote Serde attribute.
9. Add Serde getter attribute to each of fields of `TestResultDef`. Each getter attribute should contain path to the corresponding getter function of the `other_crate::TestResult` type.
10. Don't forget to tell Serde to use `TestResultDef` for serializing and deserializing `external_result` field of the `SomeStruct` struct using with attribute.
11. Add `impl From<TestResultDef>` for `other_crate::TestResult` type, so Serde will call it for deserializing.
12. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

