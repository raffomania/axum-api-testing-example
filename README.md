# Testing Your Rust Web API

A boilerplate setup with best practices for testing your axum backend that scales nicely for larger codebases with lots of test cases. Features:

- Convenient **custom HTTP client** making tests easier to write
- **Github Action** that runs tests, verifies formatting etc.

This repo is also the example code for ["End-to-end Testing Your Rust Backend"](https://www.rafa.ee/talks/end-to-end-testing-your-rust-backend/), a talk held at EuroRust 2022. If you want to know more about the techniques used here, check out the talk!

Here's a short guide to reading the code:

1. Check out the application under test in `src/lib.rs`. It's a little dog registry written using [axum](https://github.com/tokio-rs/axum) where you can insert and list your dogs using a REST API.
2. Look at `tests/dogs.rs` for two examples of how a test for this API could look. `v1` is similar to the official axum examples, while `v2` uses a test request builder to achieve the same result with less code.
3. See `tests/testing_utilities.rs` for the implementation of the request builder.
4. In `tests/generic.rs`, you can see how we can use the request builder to run the same test against multiple endpoints.

## Running the tests

In your terminal, enter:

    cargo test