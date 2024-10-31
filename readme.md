Demonstrate `serde_yml` causing a panic.  See `main.rs` for details.

1. generate a struct with the string causing the panic:

```rust
let t = Test {
    // NOTE: the amount of spaces at the end matters
    content: "\n    a {\n        ".into(),
};
```

2. serialize it using `serde_yml::to_string`

3. read back the file and attempting to deserialize it fails

- NOTE: if you modify the yaml where `content` is a raw string it works:

```yaml
# this works with and without beginning and ending quotes
content: |
  "\n    a {\n        "
```

