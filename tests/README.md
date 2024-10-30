# Test-Driven Development

Run a single test with the `--test` flag

```shell
cargo test --test tests_process_brownian
```

the above command `--test tests_process_brownian` specifies which test file to run. It corresponds to the `[[test]]` section in the `tests/Cargo.toml`, where it is defined the  `tests_process_brownian` as a test that points to the `process/brownian.rs` file. Thus, by using this command, only the tests defined in `brownian.rs` will be executed, and all other tests in the `tests/` directory will be skipped.

