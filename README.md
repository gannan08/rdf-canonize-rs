# rdf-canonize-rs

An implementation of the [RDF Dataset Normalization Algorithm][] in Rust.

## Introduction

...

## Installation

TBD

## Examples

### Use Library
```rust
let dataset_str = r#"_:b0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org webledger#ContinuityMergeEvent> .
_:b0 <https://w3id.org/security#proof> _:b1 .
_:b0 <https://w3id.org/webledger#parentHash> "zQmPkZrQs9dyezAQkVniqkMjm5nP3cdWFBzNsnnFLrsNf9u" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmYDcw6hXTZHCYaPyuGLCo8jcNREidQs4ikwKdVyS5uwKA" .
_:b0 <https://w3id.org/webledger#parentHash> "zQma45eMXmzKBXYwLdU7FvAEW3ekMy4fJjqEQVhYQFgwYAP" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmb6eicGxT6FAAZdxEzam2JpPu8ajiMJYhzPnhgHJJKh8f" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmc6b7weYQEu2NBDK9DB4HBc4bt2qQGbkvkEZBW6ajJ5F7" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmdxvSCwPjTvx3SAN2XHZ4uQpHKpbnHmns9BF8uZASW6Lx" .
_:b0 <https://w3id.org/webledger#parentHash> "zQmePs3zy2fLPEsBXqGn2LPWSGYbzPy7CZTTz1f2ng3ysph" .
_:b0 <https://w3id.org/webledger#treeHash> "zQmPkZrQs9dyezAQkVniqkMjm5nP3cdWFBzNsnnFLrsNf9u" .
_:b2 <http://purl.org/dc/terms/created> "2018-12-21T23:40:20Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> _:b1 .
_:b2 <http://purl.org/dc/terms/creator> <https://bedrock.localhost:18443/consensus/continuity2017/voters/z6MkkabTusFkLnquxwHwCm28v59UX3P9Pn5scvc7fCaNvWUL> _:b1 .
_:b2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2018> _:b1 .
_:b2 <https://w3id.org/security#jws> "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..JJ5c7mF7ru9XhPtrNqj1s6J74yqOC0HcNyK_Wa0OcfDaiODZFIJ2dXIrc_qqqvTWynIqJid6yXkKsGAzyi_HDQ" _:b1 .
"#;

let rdf_dataset = rdf_canonize::nquads::parse_nquads(&dataset_str);
let canonized_quads = rdf_canonize::canonize(&rdf_dataset, "URDNA2015").unwrap();
```

### Run Binary
```sh
git clone --depth 1 https://github.com/json-ld/normalization.git _normalization
cargo run _normalization/tests/test061-in.nq
```

## Tests
-----

```sh
cargo test
```

## Benchmark

TBD

## Source

The source code for this library is available at:

https://github.com/gannan08/rdf-canonize-rs

## Commercial Support

Commercial support for this library is available upon request from

TBD

[Digital Bazaar]: https://digitalbazaar.com/
[JSON-LD]: https://json-ld.org/
[RDF Dataset Normalization Algorithm]: https://json-ld.github.io/normalization/
[jsonld.js]: https://github.com/digitalbazaar/jsonld.js
[rdf-canonize-native]: https://github.com/digitalbazaar/rdf-canonize-native
