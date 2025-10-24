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

The CLI provides various subcommands for working with OSC data:

```bash
# Show help
osc-devtools --help

# Test roundtrip conversion between JSON and MessagePack
osc-devtools roundtrip

# Convert JSON to MessagePack
osc-devtools json-to-msgpack input.json output.msgpack

# Convert MessagePack to JSON  
osc-devtools msgpack-to-json input.msgpack output.json

# Validate OSC IR data
osc-devtools validate data.json
```

## Examples

### Roundtrip Testing

Test that data survives JSON ↔ MessagePack conversion:

```bash
# Creates test data and verifies it converts correctly
osc-devtools roundtrip
```

This creates complex nested bundle structures and verifies:
- JSON → IR → MessagePack → IR → JSON produces identical results
- Bundle structure is preserved
- All data types are handled correctly

### Format Conversion

Convert between JSON and MessagePack formats:

```bash
# JSON file containing OSC IR data
echo '{"$type": "bundle", "timetag": 12345, "elements": []}' > bundle.json

# Convert to MessagePack
osc-devtools json-to-msgpack bundle.json bundle.msgpack

# Convert back to JSON
osc-devtools msgpack-to-json bundle.msgpack restored.json

# Files should contain equivalent data
```

### Development Usage

The tools are useful for:

- **Testing codec implementations** with real data
- **Debugging serialization issues** across formats  
- **Performance benchmarking** of different formats
- **Generating test fixtures** for other projects
- **Validating OSC data structures** before processing

## Library Usage

The crate also provides library functions for programmatic use:

```rust
use osc_devtools::{create_test_bundle, roundtrip_test};

// Create complex test data
let test_data = create_test_bundle();

// Test roundtrip conversion
let success = roundtrip_test(&test_data);
assert!(success);
```

## Command Reference

### `roundtrip`
Tests bidirectional conversion between JSON and MessagePack formats using complex test data.

### `json-to-msgpack <input> <output>`
Converts JSON file to MessagePack binary format.

### `msgpack-to-json <input> <output>`  
Converts MessagePack binary to JSON format.

### `validate <input>`
Validates that input file contains valid OSC IR data.

### `benchmark`
Runs performance benchmarks comparing JSON vs MessagePack serialization.

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