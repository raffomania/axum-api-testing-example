# End-to-end Testing Your Rust Backend

This is the example code for ["End-to-end Testing Your Rust Backend"](https://www.rafa.ee/talks/end-to-end-testing-your-rust-backend/), a talk held at EuroRust 2022.

Here's a short guide to reading the code:

1. Check out the application under test in `src/lib.rs`. It's a little dog registry written using [axum](https://github.com/tokio-rs/axum) where you can insert and list your dogs using a REST API.
2. Look at `tests/dogs.rs` for two examples of how a test for this API could look. `v1` is similar to the official axum examples, while `v2` uses a test request builder to achieve the same result with less code.
3. See `tests/testing_utilities.rs` for the implementation of the request builder.
4. In `tests/generic.rs`, you can see how we can use the request builder to run the same test against multiple endpoints.