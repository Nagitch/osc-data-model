# osc-data-model (workspace)

A set of crates providing a protocol-agnostic Intermediate Representation (IR) for OSC-adjacent data and codecs to/from JSON and MessagePack, plus an adapter for `rust-osc-types`.

## Features

- **Bundle Support**: Full OSC Bundle implementation with nested bundle support
- **Cross-format Compatibility**: Seamless conversion between JSON and MessagePack  
- **Protocol Agnostic**: IR design works with any transport or encoding
- **no_std Support**: Core IR works in embedded environments (with `alloc`)

## Crates
- `osc-ir`: Minimal-dependency IR type definitions (no_std/alloc-friendly).
- `osc-codec-json`: JSON <-> IR.
- `osc-codec-msgpack`: MessagePack <-> IR.
- `osc-adapter-osc-types`: Conversions between `osc-ir` and `rust-osc-types` (1.0/1.1).
- `osc-devtools`: Small CLI for round-trips and fixtures.

## MSRV
- MSRV is **1.75**, providing access to modern Rust features and latest dependency versions. 

## License
MIT OR Apache-2.0

