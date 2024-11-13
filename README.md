# lamu

The minimal runtime for AWS Lambda.

```console
RUSTFLAGS="-C target-cpu=neoverse-n1" cross build --target aarch64-unknown-linux-gnu --release
```

```
RUSTFLAGS="-C target-cpu=neoverse-n1 -C opt-level=z -C strip=symbols" cross build --target aarch64-unknown-linux-gnu --release
```

Ref:
- https://github.com/johnthagen/min-sized-rust
