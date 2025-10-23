# osc-data-model (workspace)

A set of crates providing a protocol-agnostic Intermediate Representation (IR) for OSC-adjacent data and codecs to/from JSON and MessagePack, plus an adapter for `rust-osc-types`.

## Crates
- `osc-ir`: Minimal-dependency IR type definitions (no_std/alloc-friendly).
- `osc-codec-json`: JSON <-> IR.
- `osc-codec-msgpack`: MessagePack <-> IR.
- `osc-adapter-osc-types`: Conversions between `osc-ir` and `rust-osc-types` (1.0/1.1).
- `osc-devtools`: Small CLI for round-trips and fixtures.

## MSRV
- MSRV is **1.70**, matching `rust-osc-types`. Newer compilers are fine.

## License
MIT OR Apache-2.0

