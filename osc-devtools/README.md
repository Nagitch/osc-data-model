# osc-devtools

⚠️ **EXPERIMENTAL** ⚠️  
This crate is experimental and APIs may change significantly between versions.

CLI tools and examples for the `osc-data-model` workspace, providing utilities for testing, debugging, and demonstrating OSC intermediate representation functionality.

## Installation

```bash
cargo install osc-devtools --version 0.1.0-alpha.1
```

Or build from source:

```bash
git clone https://github.com/Nagitch/osc-data-model
cd osc-data-model
cargo build --release --bin osc-devtools
```

## Usage

The CLI provides subcommands for testing and demonstrating OSC data conversion:

```bash
# Show help
osc-devtools --help

# Test JSON roundtrip conversion
osc-devtools json-roundtrip

# Test MessagePack roundtrip conversion
osc-devtools msgpack-roundtrip

# Demonstrate complex bundle nesting and conversion
osc-devtools bundle-demo
```

## Examples

### JSON Roundtrip Testing

Test that data survives JSON serialization and deserialization:

```bash
# Test JSON roundtrip with simple string data
osc-devtools json-roundtrip
```

### MessagePack Roundtrip Testing

Test that data survives MessagePack serialization and deserialization:

```bash
# Test MessagePack roundtrip with simple string data
osc-devtools msgpack-roundtrip
```

### Bundle Demo

Demonstrate complex nested bundle structures and cross-format conversion:

```bash
# Creates complex nested bundles and tests conversion
osc-devtools bundle-demo
```

This command:
- Creates complex nested bundle structures
- Tests JSON and MessagePack roundtrip conversion
- Verifies cross-codec compatibility
- Reports conversion success and data sizes

### Development Usage

The tools are useful for:

- **Testing codec implementations** with real data
- **Debugging serialization issues** across formats  
- **Performance benchmarking** of different formats
- **Generating test fixtures** for other projects
- **Validating OSC data structures** before processing

## Command Reference

### `json-roundtrip`
Tests JSON serialization and deserialization using a simple string value.

### `msgpack-roundtrip`
Tests MessagePack serialization and deserialization using a simple string value.

### `bundle-demo`
Demonstrates complex nested bundle creation and tests both JSON and MessagePack conversion with cross-format compatibility verification.

## Dependencies

- `osc-ir`: Core IR types
- `osc-codec-json`: JSON serialization
- `osc-codec-msgpack`: MessagePack serialization
- `clap`: Command-line argument parsing
- `anyhow`: Error handling

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.