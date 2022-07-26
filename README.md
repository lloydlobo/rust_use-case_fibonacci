# Rust Use-case: Instant Fibonacci

Generate fibonacci numbers instantly.

## Development

### Run the binary

Runs the `main.rs` binary.

- To avoid stack overflow, use the release tag instead for optimization.

```sh
cargo run --release
```

- Run with watch mode on.

```sh
cargo watch -x 'run --release'
```

### Benchmark

- To Run Benchmark with Criterion.

```sh
cargo bench -p <package-name>
```

- Benchmark with Watch mode on!

```sh
cargo watch -x 'bench -p fibonacci'
```

## Dependencies

- Benchmark with Criterion

```toml
[dev-dependencies]
criterion = "0.3.6"

[[bench]]
name = "benchmarks"
harness = false
```
