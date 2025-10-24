# osc-data-model (workspace)

⚠️ **EXPERIMENTAL** ⚠️  
This is an experimental set of crates in early development. APIs are not stable and may change significantly between versions. Use with caution in production environments.

A set of crates providing a protocol-agnostic Intermediate Representation (IR) for OSC-adjacent data and codecs to/from JSON and MessagePack, plus an adapter for `rust-osc-types`.

## Features

- **OSC Version Support**: Configurable support for OSC 1.0 and OSC 1.1 via feature flags
- **Bundle Support**: Full OSC Bundle implementation with nested bundle support
- **Cross-format Compatibility**: Seamless conversion between JSON and MessagePack  
- **Protocol Agnostic**: IR design works with any transport or encoding
- **no_std Support**: Core IR works in embedded environments (with `alloc`)

## OSC Version Compatibility

The `osc-ir` crate supports different OSC protocol versions through feature flags:

- `osc10`: OSC 1.0 support (includes bundles, timetags, basic types) - **default**
- `osc11`: OSC 1.1 support (includes all OSC 1.0 features plus Color, MIDI types)

### Usage Examples

```toml
# Default: OSC 1.0 support
[dependencies]
osc-ir = "0.1.0-alpha.1"

# OSC 1.1 support
[dependencies]
osc-ir = { version = "0.1.0-alpha.1", features = ["osc11"] }

# Basic IR only (no OSC-specific features)
[dependencies]
osc-ir = { version = "0.1.0-alpha.1", default-features = false }

# With serde support for JSON/MessagePack
[dependencies]
osc-ir = { version = "0.1.0-alpha.1", features = ["osc10", "serde"] }
```

## Crates

All crates are currently in experimental alpha stage (version 0.1.0-alpha.1):

- **`osc-ir`**: Core intermediate representation types with no_std support
- **`osc-codec-json`**: JSON serialization codec for `osc-ir`
- **`osc-codec-msgpack`**: MessagePack serialization codec for `osc-ir`
- **`osc-adapter-osc-types`**: Conversions between `osc-ir` and `rust-osc-types` (disabled, TODO)
- **`osc-devtools`**: CLI tools for testing and development

## MSRV
- MSRV is **1.75**, providing access to modern Rust features and latest dependency versions. 

## License
MIT OR Apache-2.0

