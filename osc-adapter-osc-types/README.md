# osc-adapter-osc-types

⚠️ **EXPERIMENTAL** ⚠️  
This crate is experimental and APIs may change significantly between versions.

Bidirectional adapter between `osc-ir` intermediate representation and `rust-osc-types` for seamless conversion between OSC data formats.

## Features

- **Bidirectional Conversion**: Convert between `IrValue` and OSC types from `rust-osc-types`
- **OSC Version Support**: Support for both OSC 1.0 and OSC 1.1 via feature flags
- **Message Conversion**: Convert OSC messages to/from IR representation
- **Type Preservation**: Maintain type information during conversion
- **no_std Compatible**: Works in no_std environments with `alloc`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
osc-adapter-osc-types = { version = "0.1.0-alpha.1", features = ["osc10"] }
```

### OSC 1.0 Support

```toml
[dependencies]
osc-adapter-osc-types = { version = "0.1.0-alpha.1", features = ["osc10"] }
```

### OSC 1.1 Support

```toml
[dependencies]
osc-adapter-osc-types = { version = "0.1.0-alpha.1", features = ["osc11"] }
```

### Basic Example

```rust
use osc_adapter_osc_types::{osc_to_ir, ir_to_osc};
use osc_ir::IrValue;

// Convert OSC message to IR
let osc_msg = /* your OSC message */;
let ir_value = osc_to_ir(&osc_msg);

// Convert back to OSC
let restored_osc = ir_to_osc(&ir_value);
```

### Message Conversion

```rust
use osc_adapter_osc_types::{message_to_ir, ir_to_message};
use osc_ir::IrValue;

// Create an OSC message representation in IR
let address = "/oscillator/frequency";
let args = vec![
    IrValue::from(440.0),  // frequency
    IrValue::from("sine")  // waveform
];

let ir_message = message_to_ir(address, args);

// Convert IR back to OSC message format
if let Some((addr, arguments)) = ir_to_message(&ir_message) {
    println!("Address: {}", addr);
    println!("Arguments: {:?}", arguments);
}
```

### Type Conversions

The adapter handles conversion between OSC types and IR values:

- **Integers**: `i32` ↔ `IrValue::Integer`
- **Floats**: `f32` ↔ `IrValue::Float`  
- **Strings**: `String` ↔ `IrValue::String`
- **Binary Data**: `Vec<u8>` ↔ `IrValue::Binary`
- **Arrays**: OSC arrays ↔ `IrValue::Array`
- **Timestamps**: OSC timetags ↔ `IrValue::Timestamp`

## Feature Flags

- `osc10`: Enable OSC 1.0 support (basic types, bundles, timetags)
- `osc11`: Enable OSC 1.1 support (includes OSC 1.0 plus additional types)

Choose the appropriate feature flag based on the OSC version you need to support.

## API Reference

### Core Functions

- `osc_to_ir(osc: &OscType) -> IrValue` - Convert OSC type to IR
- `ir_to_osc(ir: &IrValue) -> OscType` - Convert IR to OSC type
- `message_to_ir(address: &str, args: Vec<IrValue>) -> IrValue` - Create IR message
- `ir_to_message(ir: &IrValue) -> Option<(&str, &[IrValue])>` - Extract message from IR

## Compatibility

This adapter is designed to work with:
- `osc-ir` for intermediate representation
- `rust-osc-types` for OSC protocol implementation
- Both `osc-codec-json` and `osc-codec-msgpack` for serialization

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.