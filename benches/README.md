```
cargo bench

# run single bench
cargo bench test044_canonize

# enable print/prinln
cargo bench test044_canonize -- --nocapture

# run test in benches
cargo test --release --benches test044_canonize -- --nocapture
```
