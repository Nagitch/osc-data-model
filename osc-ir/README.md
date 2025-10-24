# osc-ir

⚠️ **EXPERIMENTAL** ⚠️  
This crate is experimental and APIs may change significantly between versions.

A protocol-agnostic Intermediate Representation (IR) for OSC-adjacent data structures, designed to work seamlessly with JSON, MessagePack, and other serialization formats.

## Features

- **OSC Version Support**: Configurable OSC 1.0 and OSC 1.1 support via feature flags
- **no_std Compatible**: Core functionality works without std (requires `alloc` feature for owned containers)
- **Bundle Support**: Full OSC Bundle implementation with nested bundle support  
- **Flexible Types**: Support for all OSC types including timestamps, binary data, and extensible types
- **Serde Integration**: Optional serde support for JSON/MessagePack serialization

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
osc-ir = "0.1.0-alpha.1"
```

### Basic Example

```rust
use osc_ir::{IrValue, IrBundle, IrTimetag};

// Create basic values
let message = IrValue::from("hello world");
let number = IrValue::from(42);
let boolean = IrValue::from(true);

// Create arrays
let array = IrValue::from(vec![
    IrValue::from(1),
    IrValue::from(2), 
    IrValue::from(3)
]);

// Create bundles with timetags
let mut bundle = IrBundle::new(IrTimetag::from_ntp(12345));
bundle.add_message(message);
bundle.add_message(number);

let bundle_value = IrValue::Bundle(bundle);
```

### OSC 1.1 Features

Enable OSC 1.1 support for additional types:

```toml
[dependencies]
osc-ir = { version = "0.1.0-alpha.1", features = ["osc11"] }
```

```rust
use osc_ir::IrValue;

// OSC 1.1 Color type (RGBA)
let color = IrValue::color(255, 128, 0, 255);

// OSC 1.1 MIDI message
let midi = IrValue::midi(0, 0x90, 60, 127);
```

### Serde Support

For JSON/MessagePack serialization:

```toml
[dependencies]
osc-ir = { version = "0.1.0-alpha.1", features = ["serde"] }
```

## Feature Flags

- `alloc` (default): Enable owned containers (Vec, String, etc.) for no_std environments
- `serde`: Enable serde serialization support
- `osc10` (default): OSC 1.0 support (bundles, timetags, basic types)
- `osc11`: OSC 1.1 support (includes OSC 1.0 plus Color and MIDI types)

## no_std Support

The crate works in no_std environments with the `alloc` feature:

```toml
[dependencies]
osc-ir = { version = "0.1.0-alpha.1", default-features = false, features = ["alloc"] }
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.