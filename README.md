# Tiny Hands-on Practical Course on Serde

This is a tiny hands-on practical course on [Serde](https://serde.rs), a popular serialization and deserialization library for Rust.

The course is aimed at beginners who want to learn how to use Serde to serialize and deserialize Rust data structures.
It consists of 23 exercises, each of which is placed in its own binary package.

# Getting Started

To get started with the course, you'll need to have Rust and Cargo installed on your machine.
You can download Rust and Cargo from the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can choose the way that works best for you to get started with the course.

## Option 1: Clone the Repository

You can clone the repository and navigate to the root directory:

```bash
git clone https://github.com/YawKar/serde-course.git
cd serde-course
```

## Option 2: Fork the Repository

Alternatively, you can fork the repository and do the exercises in your own fork.
Click the "Fork" button and proceed with cloning it to your local machine:

```bash
git clone https://github.com/<your-username>/serde-course.git
cd serde-course
```

# Doing the Exercises

The whole project is a Cargo.toml with 23 workspaces corresponding to each exercise.
To do an exercise, navigate to the corresponding directory and modify the files there according to the exercise instructions:

```bash
cd ex01
# Modify the files in this directory
```

Once you have made your changes, you can use Cargo to run the exercise's tests and check if your solution produces the correct
behaviour:

```bash
cargo test
```

If the tests pass, congratulations! You have completed the exercise. If not, you can modify your solution and try again.

Each exercise has its own README.md with a task description and instructions on how to solve the exercise.

# Running Tests on GitHub Actions

If you forked the repository and want to run tests on GitHub Actions, there is already a setup workflow in the `serde-course`
repository that will run tests on GitHub runners when you push your changes to your fork. To enable the workflow, go to the
"Actions" tab on your forked repository page and enable workflows.

# Contributing

If you find any issues or have suggestions for improvement, please feel free to open an issue or a pull request on the repository.

# License

This course is licensed under the [MIT License](./LICENSE)

