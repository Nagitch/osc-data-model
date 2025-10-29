//! # osc-codec-msgpack
//!
//! ⚠️ **EXPERIMENTAL** ⚠️  
//! This crate is experimental and APIs may change significantly between versions.
//!
//! MessagePack codec for the `osc-ir` intermediate representation, enabling efficient binary
//! serialization of OSC data structures.
//!
//! ## Features
//!
//! - **Bidirectional Conversion**: Convert `IrValue` to/from MessagePack binary format
//! - **Efficient Storage**: Compact binary representation with MessagePack
//! - **Bundle Support**: Full support for OSC bundles with nested structures  
//! - **Type Preservation**: Native support for binary data, timestamps, and all OSC types
//! - **Cross-Format Compatibility**: Works seamlessly with JSON codec for the same data
//!
//! ## Usage
//!
//! ```rust
//! use osc_ir::{IrValue, IrBundle, IrTimetag};
//! use osc_codec_msgpack::{to_msgpack, from_msgpack};
//!
//! // Create some data
//! # #[cfg(feature = "osc10")]
//! # {
//! let mut bundle = IrBundle::new(IrTimetag::from_ntp(12345));
//! bundle.add_message(IrValue::from("hello"));
//! bundle.add_message(IrValue::from(42));
//! bundle.add_message(IrValue::from(vec![1u8, 2, 3, 4])); // binary data
//!
//! let value = IrValue::Bundle(bundle);
//!
//! // Convert to MessagePack
//! let msgpack_data = to_msgpack(&value);
//! println!("Serialized {} bytes", msgpack_data.len());
//!
//! // Convert back from MessagePack
//! let restored = from_msgpack(&msgpack_data);
//! assert_eq!(value, restored);
//! # }
//! ```
//!
//! ## Performance
//!
//! MessagePack typically provides:
//! - **Smaller size** than JSON (especially for binary data)
//! - **Faster serialization/deserialization** than JSON
//! - **Native binary support** without encoding overhead
//!
//! ## API Reference
//!
//! ### Core Functions
//!
//! - [`to_msgpack`] - Convert IR to MessagePack binary
//! - [`from_msgpack`] - Convert MessagePack binary to IR
//! - [`try_to_msgpack`] - Fallible conversion to MessagePack
//! - [`try_from_msgpack`] - Fallible conversion from MessagePack

use osc_ir::IrValue;

pub type EncodeResult<T> = Result<T, rmp_serde::encode::Error>;
pub type DecodeResult<T> = Result<T, rmp_serde::decode::Error>;

pub fn try_to_msgpack(v: &IrValue) -> EncodeResult<Vec<u8>> {
    rmp_serde::to_vec_named(v)
}

pub fn to_msgpack(v: &IrValue) -> Vec<u8> {
    try_to_msgpack(v).expect("serialize")
}

pub fn try_from_msgpack(bytes: &[u8]) -> DecodeResult<IrValue> {
    rmp_serde::from_slice::<IrValue>(bytes)
}

pub fn from_msgpack(bytes: &[u8]) -> IrValue {
    try_from_msgpack(bytes).expect("deserialize")
}

#[cfg(test)]
mod tests {
    use super::*;
    use osc_ir::{IrTimestamp, IrBundle, IrTimetag};

    #[test]
    fn roundtrip_timestamp() {
        let value = IrValue::from(IrTimestamp {
            seconds: 123,
            nanos: 456,
        });
        let bytes = try_to_msgpack(&value).expect("encode");
        let decoded = try_from_msgpack(&bytes).expect("decode");
        assert_eq!(value, decoded);
    }

    #[test]
    fn roundtrip_complex_structure() {
        let value = IrValue::Map(vec![
            ("msg".into(), IrValue::from("hello")),
            ("bin".into(), IrValue::from(vec![1_u8, 2, 3])),
            (
                "ext".into(),
                IrValue::Ext {
                    type_id: -4,
                    data: vec![0x10, 0x20, 0x30],
                },
            ),
        ]);

        let bytes = to_msgpack(&value);
        let decoded = from_msgpack(&bytes);
        assert_eq!(decoded, value);
    }

    #[test]
    fn roundtrip_bundle() {
        let mut bundle = IrBundle::new(IrTimetag::from_ntp(12345));
        bundle.add_message(IrValue::from("hello"));
        bundle.add_message(IrValue::from(42));

        let mut nested_bundle = IrBundle::immediate();
        nested_bundle.add_message(IrValue::from(true));
        
        bundle.add_bundle(nested_bundle);

        let value = IrValue::Bundle(bundle);
        let bytes = to_msgpack(&value);
        let decoded = from_msgpack(&bytes);
        assert_eq!(decoded, value);
    }

    #[test]
    fn roundtrip_deeply_nested_bundle() {
        // Create a deeply nested bundle structure
        let mut root = IrBundle::immediate();
        root.add_message(IrValue::from("root"));

        let mut level1 = IrBundle::new(IrTimetag::from_ntp(1000));
        level1.add_message(IrValue::from("level1"));

        let mut level2 = IrBundle::new(IrTimetag::from_ntp(2000));
        level2.add_message(IrValue::from("level2"));

        level1.add_bundle(level2);
        root.add_bundle(level1);

        let value = IrValue::Bundle(root);

        // Test roundtrip
        let bytes = to_msgpack(&value);
        let decoded = from_msgpack(&bytes);
        assert_eq!(value, decoded);
    }

    #[test]
    fn roundtrip_osc_message_like() {
        // Construct an IrValue that matches the adapter's OSC message representation
        let value = IrValue::Map(vec![
            ("$type".into(), IrValue::from("osc.message")),
            ("address".into(), IrValue::from("/test")),
            (
                "args".into(),
                IrValue::Array(vec![
                    IrValue::Integer(7),
                    IrValue::Float(1.5),
                    IrValue::from("text"),
                    IrValue::Binary(vec![1_u8, 2, 3]),
                ]),
            ),
        ]);

        let bytes = to_msgpack(&value);
        let decoded = from_msgpack(&bytes);

        // Structure-preserving roundtrip
        assert_eq!(decoded, value);

        // Extract and validate fields similar to adapter::try_extract_message
        let map = decoded.as_map().expect("expected map");
        let address = map.iter().find(|(k, _)| k == "address").unwrap().1.as_str();
        assert_eq!(address, Some("/test"));

        let args = map
            .iter()
            .find(|(k, _)| k == "args")
            .unwrap()
            .1
            .as_array()
            .expect("expected args array");

        assert_eq!(args.len(), 4);
        assert_eq!(args[0].as_integer(), Some(7));
        assert!((args[1].as_float().unwrap() - 1.5).abs() < f64::EPSILON);
        assert_eq!(args[2].as_str(), Some("text"));
        assert_eq!(args[3].as_binary(), Some(&[1_u8, 2, 3][..]));
    }

    #[test]
    fn msgpack_bytes_are_valid_and_match_contents() {
        use std::io::Cursor;
        use rmpv::{decode::read_value, Value};

        // Prepare an OSC-like message map as IrValue
        let value = IrValue::Map(vec![
            ("$type".into(), IrValue::from("osc.message")),
            ("address".into(), IrValue::from("/validate")),
            (
                "args".into(),
                IrValue::Array(vec![
                    IrValue::Integer(123),
                    IrValue::Float(-2.5),
                    IrValue::from("ok"),
                    IrValue::Binary(vec![0xAA, 0xBB]),
                ]),
            ),
        ]);

        // Encode to MessagePack
        let bytes = to_msgpack(&value);

        // Ensure bytes are valid MessagePack by decoding with rmpv
        let mut cursor = Cursor::new(&bytes);
        let root = read_value(&mut cursor).expect("must decode as msgpack Value");

        // Helper: unwrap serde's externally tagged enum representation
        fn unwrap_enum(v: &Value) -> (&str, &Value) {
            match v {
                // Map form: { "Variant": payload }
                Value::Map(kv) if kv.len() == 1 => {
                    let (k, v) = &kv[0];
                    let name = match k { Value::String(s) => s.as_str().expect("variant name"), _ => panic!("invalid enum key") };
                    (name, v)
                }
                // Array form: ["Variant", payload]
                Value::Array(items) if items.len() == 2 => {
                    let name = match &items[0] { Value::String(s) => s.as_str().expect("variant name"), _ => panic!("invalid enum tag array") };
                    (name, &items[1])
                }
                other => panic!("unexpected enum encoding: {:?}", other),
            }
        }

        // Root must be the IrValue::Map(enum) variant
        let (root_variant, root_payload) = unwrap_enum(&root);
        assert_eq!(root_variant, "Map");

        // Payload is Vec<(String, IrValue)> serialized as array of 2-element arrays
        let entries = match root_payload { Value::Array(a) => a, other => panic!("expected entries array, got {:?}", other) };

        // Collect into hashmap-like view: key -> encoded IrValue
    let get = |key: &str| -> &Value {
            entries
                .iter()
                .find_map(|entry| match entry {
                    Value::Array(items) if items.len() == 2 => match (&items[0], &items[1]) {
                        (Value::String(s), v) if s.as_str() == Some(key) => Some(v),
                        _ => None,
                    },
                    _ => None,
                })
                .expect("entry not found")
        };

        // $type: IrValue::String("osc.message") -> enum String with payload string
        let (ty_variant, ty_payload) = unwrap_enum(get("$type"));
        assert_eq!(ty_variant, "String");
        assert!(matches!(ty_payload, Value::String(s) if s.as_str() == Some("osc.message")));

        // address: IrValue::String("/validate")
        let (addr_variant, addr_payload) = unwrap_enum(get("address"));
        assert_eq!(addr_variant, "String");
        assert!(matches!(addr_payload, Value::String(s) if s.as_str() == Some("/validate")));

        // args: IrValue::Array([...]) -> enum Array with payload array of encoded IrValue
        let (args_variant, args_payload) = unwrap_enum(get("args"));
        assert_eq!(args_variant, "Array");
        let args = match args_payload { Value::Array(a) => a, other => panic!("expected args payload array, got {:?}", other) };
        assert_eq!(args.len(), 4);

        // 0: Integer(123)
        let (v0_variant, v0_payload) = unwrap_enum(&args[0]);
        assert_eq!(v0_variant, "Integer");
        assert!(matches!(v0_payload, Value::Integer(i) if i.as_i64() == Some(123)));

        // 1: Float(-2.5)
        let (v1_variant, v1_payload) = unwrap_enum(&args[1]);
        assert_eq!(v1_variant, "Float");
        assert!(matches!(v1_payload, Value::F64(x) if (*x + 2.5).abs() < f64::EPSILON));

        // 2: String("ok")
        let (v2_variant, v2_payload) = unwrap_enum(&args[2]);
        assert_eq!(v2_variant, "String");
        assert!(matches!(v2_payload, Value::String(s) if s.as_str() == Some("ok")));

        // 3: Binary([0xAA, 0xBB])
        let (v3_variant, v3_payload) = unwrap_enum(&args[3]);
        assert_eq!(v3_variant, "Binary");
        assert!(matches!(v3_payload, Value::Binary(b) if b.as_slice() == [0xAA, 0xBB]));
    }
}
