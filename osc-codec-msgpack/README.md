# osc-codec-msgpack

⚠️ **EXPERIMENTAL** ⚠️  
This crate is experimental and APIs may change significantly between versions.

MessagePack codec for the `osc-ir` intermediate representation, enabling efficient binary serialization of OSC data structures.

## Features

- **Bidirectional Conversion**: Convert `IrValue` to/from MessagePack binary format
- **Efficient Storage**: Compact binary representation with MessagePack
- **Bundle Support**: Full support for OSC bundles with nested structures  
- **Type Preservation**: Native support for binary data, timestamps, and all OSC types
- **Cross-Format Compatibility**: Works seamlessly with JSON codec for the same data

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
osc-codec-msgpack = "0.1.0-alpha.1"
```

### Basic Example

```rust
use osc_ir::{IrValue, IrBundle, IrTimetag};
use osc_codec_msgpack::{to_msgpack, from_msgpack};

// Create some data
let mut bundle = IrBundle::new(IrTimetag::from_ntp(12345));
bundle.add_message(IrValue::from("hello"));
bundle.add_message(IrValue::from(42));
bundle.add_message(IrValue::from(vec![1u8, 2, 3, 4])); // binary data

let value = IrValue::Bundle(bundle);

// Convert to MessagePack
let msgpack_data = to_msgpack(&value);
println!("Serialized {} bytes", msgpack_data.len());

// Convert back from MessagePack
let restored = from_msgpack(&msgpack_data);
assert_eq!(value, restored);
```

### Cross-Codec Compatibility

MessagePack and JSON codecs produce equivalent results:

```rust
use osc_ir::IrValue;
use osc_codec_json::{to_json, from_json};
use osc_codec_msgpack::{to_msgpack, from_msgpack};

let original = IrValue::from(vec![
    IrValue::from("test"),
    IrValue::from(42),
    IrValue::from(true)
]);

// Both codecs produce equivalent results
let from_json = from_json(&to_json(&original));
let from_msgpack = from_msgpack(&to_msgpack(&original));

assert_eq!(original, from_json);
assert_eq!(original, from_msgpack);
assert_eq!(from_json, from_msgpack);
```

### Complex Nested Bundles

MessagePack efficiently handles deeply nested bundle structures:

```rust
let mut root = IrBundle::immediate();
root.add_message(IrValue::from("root level"));

let mut level1 = IrBundle::new(IrTimetag::from_ntp(1000));
level1.add_message(IrValue::from(42));

let mut level2 = IrBundle::new(IrTimetag::from_ntp(2000));
level2.add_message(IrValue::from(true));
level2.add_message(IrValue::from(vec![0xAA_u8, 0xBB, 0xCC]));

level1.add_bundle(level2);
root.add_bundle(level1);

let msgpack_data = to_msgpack(&IrValue::Bundle(root));
// Efficiently serialized with preserved structure
```

### Binary Data Handling

MessagePack natively supports binary data without base64 encoding:

```rust
let binary_data = IrValue::Binary(vec![0; 1024]); // 1KB of data
let msgpack = to_msgpack(&binary_data);
// No base64 overhead - stored as native MessagePack binary
```

## Performance

MessagePack typically provides:
- **Smaller size** than JSON (especially for binary data)
- **Faster serialization/deserialization** than JSON
- **Native binary support** without encoding overhead

## API Reference

### Functions

- `to_msgpack(value: &IrValue) -> Vec<u8>` - Convert IR to MessagePack binary
- `from_msgpack(data: &[u8]) -> IrValue` - Convert MessagePack binary to IR

## Error Handling

Functions currently use `.unwrap()` for simplicity but will be improved with proper error handling in future versions.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.