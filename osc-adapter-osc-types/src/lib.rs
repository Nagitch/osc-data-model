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

    fn ir_to_arg<'a>(value: &'a IrValue) -> Option<osc::OscType<'a>> {
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

    pub fn ir_to_message<'a>(value: &'a IrValue) -> Option<osc::Message<'a>> {
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

    fn ir_to_arg<'a>(value: &'a IrValue) -> Option<osc::OscType<'a>> {
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

    pub fn ir_to_message<'a>(value: &'a IrValue) -> Option<osc::Message<'a>> {
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
