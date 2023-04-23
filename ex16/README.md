# Exercise 16: Serializing Enum Variants with Different Names

In this exercise, you'll use Serde attribute `#[serde(rename = "...")]` to serialize enum variants with different names.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `Nomenclature` enum. It's already setup for serializing and deserializing.
3. We need Serde to:
   1. Serialize and deserialize `InsideName1` variant with name `outer_variant_name_1`
   2. Serialize `InsideName2` variant with name `to_outer_variant_name_2`
   3. But deserialize the same `InsideName2` variant with name `from_outer_variant_name_2`
4. Consider test cases in the `tests` module and notice that:
   - `InsideName1` should be serializable only into `outer_variant_name_1`
   - `InsideName1` should be deserializable only from `outer_variant_name_1`
   - `InsideName2` should be serializable only into `to_outer_variant_name_2`
   - `InsideName2` should be deserializable only from `from_outer_variant_name_2`
   - `InsideName3` should keep its original name in serialization/deserialization
5. Read the description of [rename attribute](https://serde.rs/variant-attrs.html#rename)
6. Try to implement this attribute for `InsideName1` and `InsideName2` variants of the `Nomenclature` enum.
7. Once you've implemented serialization and deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

