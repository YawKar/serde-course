# Exercise 14: Deserialize Type Through Converting from Already Deserializable

In this exercise, you'll use Serde attributes `#[serde(from = "...")]` to deserialize type by deserializing into another one and then converting result into the target type.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `VeryBigStruct` struct, notice that it is already setup for serializing/deserializing.
3. Explore the `TinyStruct` struct.
4. We want Serde to make these steps in deserialization of a `TinyStruct` instance:
   1. Firstly, deserialize input into `VeryBigStruct`
   2. Secondly, convert the resulting `VeryBigStruct` instance into `TinyStruct` using `From<VeryBigStruct>` impl of `TinyStruct` 
5. Consider test cases in the `tests` module and notice that:
   - representation of `VeryBigStruct` was deserialized into `TinyStruct` instance
6. Read the description of [from attribute](https://serde.rs/container-attrs.html#from)
7. Try to implement this attribute for `TinyStruct` struct.
8. Also you need to implement `From<VeryBigStruct>` trait for `TinyStruct`. This is the requirement of the from attribute.
9. Once you've implemented deserialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

