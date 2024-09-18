# cgroup-memory

A Rust crate for reading and interpreting `/sys/fs/cgroup` memory statistics.

```bash
/sys/fs/cgroup/memory.stat
/sys/fs/cgroup/memory.max
```

## Examples

### Read total / max memory
```rust
match memory_max() {
    Ok(Some(v)) => println!("Max memory: {v}"),
    Ok(None) => println!("No max memory constraint"),
    Err(e) => println!("Failed to read and parse memory files: {e}"),
}
```

### Calculate available memory
```rust
match memory_available() {
    Ok(Some(v)) => println!("Available memory: {v} bytes"),
    Ok(None) => println!("No memory limit set"),
    Err(e) => println!("Failed to read memory information: {}", e),
}
```

## Development

### Integration tests
These tests require environments with specific memory attributes. We utilize Docker to create such virtual Linux environments.

Note that `memory.max` only contains a valid value if the `--memory` option was specified.

#### Run
```bash
docker build -t cgroup-memory
# Run tests that expect 100mb of total memory
docker run --rm --memory=100m cgroup-memory /usr/local/bin/cargo-nextest ntr --archive-file test_memory_max_100m.tar.zst --workspace-remap . --no-capture
```