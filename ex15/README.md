# Exercise 15: Serialize Type Through Converting It Into Already Serializable

In this exercise, you'll use Serde attribute `#[serde(into = "...")]` to serialize type by converting it into another one and then serializing the result.

# Task Description

1. Open the `src/main.rs` file in your preferred code editor.
2. Explore the `ResponseFromMainServer` struct, notice that it is already setup for serializing.
3. Notice the `default_main_server_key` static method of `ResponseFromMainServer`. It will be useful for conversion.
4. Explore the `ResponseFromSubServer` struct. It has impl of `Into<ResponseFromMainServer>` trait.
5. We want Serde to make these steps in serialization of a `ResponseFromSubServer` instance:
   1. Firstly, convert it into `ResponseFromMainServer` using `Into<ResponseFromMainServer>` trait impl
   2. Secondly, serialize the resulting `ResponseFromMainServer` instance
6. Consider test cases in the `tests` module and notice that:
   - `ResponseFromSubServer` instance should be correctly serialized into `ResponseFromMainServer` representation
7. Read the description of [into attribute](https://serde.rs/container-attrs.html#into)
8. Try to implement this attribute for `ResponseFromSubServer` struct.
9. Notice that you don't need to implement or change current implementation of `Into<ResponseFromMainServer>` trait!
10. Also you will need to derive/implement `Clone` trait for the `ResponseFromSubServer` trait in order for into attribute to work.
11. Once you've implemented serialization, run the tests: `cargo test`

# Additional Resources

* [Serde documentation](https://serde.rs/)
* [Unit testing in Rust](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
* [Rust documentation](https://www.rust-lang.org/learn)

