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

## Knowledge Base

### Write output or return results to file

```rs
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let data = b"some bytes";
    let mut pos = 0;
    let mut buffer = File::create("foo.txt")?;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}
```
