# osc-codec-json

⚠️ **EXPERIMENTAL** ⚠️  
This crate is experimental and APIs may change significantly between versions.

JSON codec for the `osc-ir` intermediate representation, enabling seamless conversion between OSC data structures and JSON format.

## Features

- **Bidirectional Conversion**: Convert `IrValue` to/from JSON
- **Bundle Support**: Full support for OSC bundles with nested structures
- **Type Preservation**: Special handling for binary data, timestamps, and extended types
- **OSC Compatibility**: Support for OSC 1.0 and 1.1 features via feature flags

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
osc-codec-json = "0.1.0-alpha.1"
```

### Basic Example

```rust
use osc_ir::{IrValue, IrBundle, IrTimetag};
use osc_codec_json::{to_json, from_json};

// Create some data
let mut bundle = IrBundle::new(IrTimetag::from_ntp(12345));
bundle.add_message(IrValue::from("hello"));
bundle.add_message(IrValue::from(42));

let value = IrValue::Bundle(bundle);

// Convert to JSON
let json = to_json(&value);
println!("{}", serde_json::to_string_pretty(&json).unwrap());

// Convert back from JSON
let restored = from_json(&json);
assert_eq!(value, restored);
```

### Special Type Handling

The codec handles OSC-specific types with special JSON representations:

#### Binary Data
```rust
let binary = IrValue::Binary(vec![0xAA, 0xBB, 0xCC]);
let json = to_json(&binary);
// Results in: {"$type": "binary", "data": "qrvM"}
```

#### Timestamps
```rust
let timestamp = IrValue::from(osc_ir::IrTimestamp {
    seconds: 1234567890,
    nanos: 500_000_000
});
let json = to_json(&timestamp);
// Results in: {"$type": "timestamp", "seconds": 1234567890, "nanos": 500000000}
```

#### Extended Types
```rust
let ext = IrValue::Ext {
    type_id: 42,
    data: vec![1, 2, 3, 4]
};
let json = to_json(&ext);
// Results in: {"$type": "ext", "ext": 42, "data": "AQIDBA=="}
```

### OSC Bundles

Bundles are represented with nested structure preservation:

```rust
let mut root = IrBundle::immediate();
root.add_message(IrValue::from("root message"));

let mut nested = IrBundle::new(IrTimetag::from_ntp(1000));
nested.add_message(IrValue::from("nested message"));
root.add_bundle(nested);

let json = to_json(&IrValue::Bundle(root));
// Results in properly nested JSON structure with timetags
```

## Feature Flags

- `osc10` (default): OSC 1.0 support
- `osc11`: OSC 1.1 support (includes Color and MIDI types)

Note: OSC 1.1 Color and MIDI types are currently marked as TODO and serialize to `null`.

## API Reference

### Functions

- `to_json(value: &IrValue) -> serde_json::Value` - Convert IR to JSON
- `from_json(json: &serde_json::Value) -> IrValue` - Convert JSON to IR

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.