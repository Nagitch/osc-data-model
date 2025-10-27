//! # osc-adapter-osc-types
//!
//! ⚠️ **EXPERIMENTAL** ⚠️  
//! This crate is experimental and APIs may change significantly between versions.
//!
//! Bidirectional adapter between `osc-ir` intermediate representation and `rust-osc-types`
//! for seamless conversion between OSC data formats.
//!
//! ## Features
//!
//! - **Bidirectional Conversion**: Convert between `IrValue` and OSC types from `rust-osc-types`
//! - **OSC Version Support**: Support for both OSC 1.0 and OSC 1.1 via feature flags
//! - **Message Conversion**: Convert OSC messages to/from IR representation
//! - **Type Preservation**: Maintain type information during conversion
//! - **no_std Compatible**: Works in no_std environments with `alloc`
//!
//! ## Usage
//!
//! ```rust
//! # #[cfg(any(feature = "osc10", feature = "osc11"))]
//! # {
//! use osc_adapter_osc_types::{message_to_ir, ir_to_message};
//! use osc_ir::IrValue;
//!
//! // Create an OSC message representation in IR
//! let address = "/oscillator/frequency";
//! let args = vec![
//!     IrValue::from(440.0),  // frequency
//!     IrValue::from("sine")  // waveform
//! ];
//!
//! let ir_message = message_to_ir(address, args);
//!
//! // Convert IR back to OSC message format
//! if let Some((addr, arguments)) = ir_to_message(&ir_message) {
//!     println!("Address: {}", addr);
//!     println!("Arguments: {:?}", arguments);
//! }
//! # }
//! ```

#![cfg_attr(not(test), no_std)]

extern crate alloc;

#[cfg(any(feature = "osc10", feature = "osc11"))]
use alloc::{string::String, vec, vec::Vec};
#[cfg(any(feature = "osc10", feature = "osc11"))]
use osc_ir::IrValue;

#[cfg(any(feature = "osc10", feature = "osc11"))]
const MESSAGE_TYPE_TAG: &str = "osc.message";

#[cfg(any(feature = "osc10", feature = "osc11"))]
fn message_to_ir_map(address: &str, args: Vec<IrValue>) -> IrValue {
    IrValue::Map(vec![
        (String::from("$type"), IrValue::from(MESSAGE_TYPE_TAG)),
        (String::from("address"), IrValue::from(address)),
        (String::from("args"), IrValue::Array(args)),
    ])
}

#[cfg(any(feature = "osc10", feature = "osc11"))]
fn try_extract_message<'a>(value: &'a IrValue) -> Option<(&'a str, &'a [IrValue])> {
    let map = value.as_map()?;
    let mut address: Option<&'a str> = None;
    let mut args: Option<&'a [IrValue]> = None;
    let mut has_type_tag = false;

    for (key, entry) in map.iter() {
        match key.as_str() {
            "$type" => {
                let tag = entry.as_str()?;
                if tag != MESSAGE_TYPE_TAG {
                    return None;
                }
                has_type_tag = true;
            }
            "address" => {
                address = entry.as_str();
            }
            "args" => {
                args = entry.as_array();
            }
            _ => {}
        }
    }

    if map.iter().any(|(k, _)| k == "$type") && !has_type_tag {
        return None;
    }

    let address = address?;
    let args = args.unwrap_or(&[]);
    Some((address, args))
}

#[cfg(feature = "osc10")]
pub mod v10 {
    use super::*;
    use osc_types10 as osc;

    fn arg_to_ir(arg: &osc::OscType) -> IrValue {
        match arg {
            osc::OscType::Int(v) => IrValue::Integer(*v as i64),
            osc::OscType::Float(v) => IrValue::Float(*v as f64),
            osc::OscType::String(s) => IrValue::from(*s),
            osc::OscType::Blob(bytes) => IrValue::Binary(bytes.to_vec()),
        }
    }

    fn ir_to_arg(value: &IrValue) -> Option<osc::OscType<'_>> {
        match value {
            IrValue::Integer(i) => i32::try_from(*i).ok().map(osc::OscType::Int),
            IrValue::Float(f) => Some(osc::OscType::Float(*f as f32)),
            IrValue::String(s) => Some(osc::OscType::String(s.as_ref())),
            IrValue::Binary(bytes) => Some(osc::OscType::Blob(bytes.as_slice())),
            _ => None,
        }
    }

    pub fn message_to_ir(message: &osc::Message) -> IrValue {
        let args = message.args.iter().map(arg_to_ir).collect::<Vec<_>>();
        message_to_ir_map(message.address, args)
    }

    pub fn ir_to_message(value: &IrValue) -> Option<osc::Message<'_>> {
        let (address, args) = try_extract_message(value)?;
        let mut osc_args = Vec::with_capacity(args.len());
        for arg in args {
            osc_args.push(ir_to_arg(arg)?);
        }
        Some(osc::Message {
            address,
            args: osc_args,
        })
    }
}

#[cfg(feature = "osc11")]
pub mod v11 {
    use super::*;
    use osc_types11 as osc;

    fn arg_to_ir(arg: &osc::OscType) -> IrValue {
        match arg {
            osc::OscType::Int(v) => IrValue::Integer(*v as i64),
            osc::OscType::Float(v) => IrValue::Float(*v as f64),
            osc::OscType::String(s) => IrValue::from(*s),
            osc::OscType::Blob(bytes) => IrValue::Binary(bytes.to_vec()),
        }
    }

    fn ir_to_arg(value: &IrValue) -> Option<osc::OscType<'_>> {
        match value {
            IrValue::Integer(i) => i32::try_from(*i).ok().map(osc::OscType::Int),
            IrValue::Float(f) => Some(osc::OscType::Float(*f as f32)),
            IrValue::String(s) => Some(osc::OscType::String(s.as_ref())),
            IrValue::Binary(bytes) => Some(osc::OscType::Blob(bytes.as_slice())),
            IrValue::Color { .. } | IrValue::Midi { .. } => None,
            _ => None,
        }
    }

    pub fn message_to_ir(message: &osc::Message) -> IrValue {
        let args = message.args.iter().map(arg_to_ir).collect::<Vec<_>>();
        message_to_ir_map(message.address, args)
    }

    pub fn ir_to_message(value: &IrValue) -> Option<osc::Message<'_>> {
        let (address, args) = try_extract_message(value)?;
        let mut osc_args = Vec::with_capacity(args.len());
        for arg in args {
            osc_args.push(ir_to_arg(arg)?);
        }
        Some(osc::Message {
            address,
            args: osc_args,
        })
    }
}

#[cfg(all(test, any(feature = "osc10", feature = "osc11")))]
mod tests {
    use super::*;

    #[cfg(feature = "osc10")]
    mod osc10 {
        use super::*;
        use alloc::borrow::ToOwned;

        #[test]
        fn message_to_ir_encodes_metadata_and_args() {
            use osc_types10 as osc;

            let message = osc::Message {
                address: "/basic",
                args: vec![
                    osc::OscType::Int(42),
                    osc::OscType::Float(0.5),
                    osc::OscType::String("text"),
                    osc::OscType::Blob(&[1, 2, 3]),
                ],
            };

            let ir = v10::message_to_ir(&message);
            let entries = match ir {
                IrValue::Map(entries) => entries,
                _ => panic!("expected map"),
            };

            assert_eq!(entries.len(), 3);

            let ty = entries.iter().find(|(key, _)| key == "$type").unwrap();
            assert_eq!(ty.1, IrValue::from(MESSAGE_TYPE_TAG));

            let address = entries.iter().find(|(key, _)| key == "address").unwrap();
            assert_eq!(address.1, IrValue::from("/basic"));

            let args_entry = entries.iter().find(|(key, _)| key == "args").unwrap();
            let args = match &args_entry.1 {
                IrValue::Array(values) => values,
                _ => panic!("expected args array"),
            };

            let expected = vec![
                IrValue::Integer(42),
                IrValue::Float(0.5),
                IrValue::from("text"),
                IrValue::Binary(vec![1, 2, 3]),
            ];
            assert_eq!(args, &expected);
        }

        #[test]
        fn ir_roundtrips_to_message() {
            use osc_types10 as osc;

            let ir = IrValue::Map(vec![
                ("$type".to_owned(), IrValue::from(MESSAGE_TYPE_TAG)),
                ("address".to_owned(), IrValue::from("/roundtrip")),
                (
                    "args".to_owned(),
                    IrValue::Array(vec![
                        IrValue::Integer(7),
                        IrValue::Float(-1.25),
                        IrValue::from("value"),
                        IrValue::Binary(vec![9, 8, 7]),
                    ]),
                ),
            ]);

            let message = v10::ir_to_message(&ir).expect("expected successful conversion");
            assert_eq!(message.address, "/roundtrip");
            assert!(matches!(message.args[0], osc::OscType::Int(7)));
            assert!(
                matches!(message.args[1], osc::OscType::Float(f) if (f + 1.25).abs() < f32::EPSILON)
            );
            assert!(matches!(message.args[2], osc::OscType::String("value")));
            assert!(matches!(message.args[3], osc::OscType::Blob(slice) if slice == [9, 8, 7]));
        }

        #[test]
        fn ir_to_message_rejects_unknown_arguments() {
            let ir = IrValue::Map(vec![
                ("address".to_owned(), IrValue::from("/invalid")),
                ("args".to_owned(), IrValue::Array(vec![IrValue::Bool(true)])),
            ]);

            assert!(v10::ir_to_message(&ir).is_none());
        }
    }

    #[test]
    fn mismatched_type_tag_is_rejected() {
        let value = IrValue::Map(vec![
            ("$type".into(), IrValue::from("osc.bundle")),
            ("address".into(), IrValue::from("/bad")),
            ("args".into(), IrValue::Array(vec![])),
        ]);

        assert!(try_extract_message(&value).is_none());
    }

    #[test]
    fn missing_type_tag_defaults_to_message() {
        let value = IrValue::Map(vec![
            ("address".into(), IrValue::from("/no-tag")),
            ("args".into(), IrValue::Array(vec![])),
        ]);

        let extracted = try_extract_message(&value).expect("expected message extraction");
        assert_eq!(extracted.0, "/no-tag");
        assert!(extracted.1.is_empty());
    }

    #[cfg(feature = "osc11")]
    mod osc11 {
        use super::*;

        #[test]
        fn ir_to_message_rejects_color_and_midi() {
            let ir = IrValue::Map(vec![
                ("address".into(), IrValue::from("/unsupported")),
                (
                    "args".into(),
                    IrValue::Array(vec![
                        IrValue::Color {
                            r: 0,
                            g: 1,
                            b: 2,
                            a: 3,
                        },
                        IrValue::Midi {
                            port: 1,
                            status: 2,
                            data1: 3,
                            data2: 4,
                        },
                    ]),
                ),
            ]);

            assert!(v11::ir_to_message(&ir).is_none());
        }
    }
}
