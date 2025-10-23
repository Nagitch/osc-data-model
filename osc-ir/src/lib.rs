#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::{boxed::Box, string::String, vec, vec::Vec};
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// MessagePack-friendly timestamp; interoperable with JSON via RFC3339 if needed.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IrTimestamp {
    pub seconds: i64,
    pub nanos: u32,
}

/// Protocol-agnostic value space.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum IrValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(Box<str>),
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
    Binary(Vec<u8>),
    Array(Vec<IrValue>),
    /// Map keys are Strings for JSON compatibility.
    Map(Vec<(String, IrValue)>),
    Timestamp(IrTimestamp),
    /// MessagePack Ext type compatibility; also useful to carry OSC-specific tags.
    Ext {
        type_id: i8,
        #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
        data: Vec<u8>,
    },
}

impl fmt::Display for IrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IrValue {
    pub fn null() -> Self {
        IrValue::Null
    }

    pub fn is_null(&self) -> bool {
        matches!(self, IrValue::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            IrValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            IrValue::Integer(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            IrValue::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            IrValue::String(v) => Some(v.as_ref()),
            _ => None,
        }
    }

    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            IrValue::Binary(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[IrValue]> {
        match self {
            IrValue::Array(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&[(String, IrValue)]> {
        match self {
            IrValue::Map(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<&IrTimestamp> {
        match self {
            IrValue::Timestamp(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_ext(&self) -> Option<(i8, &[u8])> {
        match self {
            IrValue::Ext { type_id, data } => Some((*type_id, data.as_slice())),
            _ => None,
        }
    }
}

impl Default for IrValue {
    fn default() -> Self {
        IrValue::Null
    }
}

impl From<()> for IrValue {
    fn from(_: ()) -> Self {
        IrValue::Null
    }
}

impl From<bool> for IrValue {
    fn from(v: bool) -> Self {
        IrValue::Bool(v)
    }
}

impl From<i8> for IrValue {
    fn from(v: i8) -> Self {
        IrValue::Integer(v as i64)
    }
}

impl From<i16> for IrValue {
    fn from(v: i16) -> Self {
        IrValue::Integer(v as i64)
    }
}

impl From<i32> for IrValue {
    fn from(v: i32) -> Self {
        IrValue::Integer(v as i64)
    }
}

impl From<i64> for IrValue {
    fn from(v: i64) -> Self {
        IrValue::Integer(v)
    }
}

impl From<isize> for IrValue {
    fn from(v: isize) -> Self {
        IrValue::Integer(v as i64)
    }
}

impl From<u8> for IrValue {
    fn from(v: u8) -> Self {
        IrValue::Integer(v as i64)
    }
}

impl From<u16> for IrValue {
    fn from(v: u16) -> Self {
        IrValue::Integer(v as i64)
    }
}

impl From<u32> for IrValue {
    fn from(v: u32) -> Self {
        IrValue::Integer(v as i64)
    }
}

impl From<f32> for IrValue {
    fn from(v: f32) -> Self {
        IrValue::Float(v as f64)
    }
}

impl From<f64> for IrValue {
    fn from(v: f64) -> Self {
        IrValue::Float(v)
    }
}

impl From<String> for IrValue {
    fn from(v: String) -> Self {
        IrValue::String(v.into_boxed_str())
    }
}

impl From<Box<str>> for IrValue {
    fn from(v: Box<str>) -> Self {
        IrValue::String(v)
    }
}

impl From<&str> for IrValue {
    fn from(v: &str) -> Self {
        IrValue::String(v.into())
    }
}

impl From<Vec<u8>> for IrValue {
    fn from(v: Vec<u8>) -> Self {
        IrValue::Binary(v)
    }
}

impl From<&[u8]> for IrValue {
    fn from(v: &[u8]) -> Self {
        IrValue::Binary(v.to_vec())
    }
}

impl From<Vec<IrValue>> for IrValue {
    fn from(v: Vec<IrValue>) -> Self {
        IrValue::Array(v)
    }
}

impl From<Vec<(String, IrValue)>> for IrValue {
    fn from(v: Vec<(String, IrValue)>) -> Self {
        IrValue::Map(v)
    }
}

impl From<IrTimestamp> for IrValue {
    fn from(v: IrTimestamp) -> Self {
        IrValue::Timestamp(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn conversions_work() {
        assert_eq!(IrValue::from(true).as_bool(), Some(true));
        assert_eq!(IrValue::from(42_i32).as_integer(), Some(42));
        assert_eq!(IrValue::from(3.5_f32).as_float(), Some(3.5));
        assert_eq!(IrValue::from("hi").as_str(), Some("hi"));
        assert_eq!(IrValue::from(vec![1_u8, 2]).as_binary(), Some(&[1, 2][..]));
        let arr = IrValue::from(vec![IrValue::from(1_i32), IrValue::from(2_i32)]);
        assert_eq!(arr.as_array().unwrap().len(), 2);
        let ts = IrTimestamp {
            seconds: 1,
            nanos: 2,
        };
        assert_eq!(IrValue::from(ts).as_timestamp(), Some(&ts));
    }

    #[test]
    fn ext_and_default_helpers() {
        let ext = IrValue::Ext {
            type_id: 9,
            data: vec![0xAA, 0xBB],
        };
        assert_eq!(ext.as_ext(), Some((9, &[0xAA, 0xBB][..])));

        let default = IrValue::default();
        assert!(default.is_null());
        assert!(default.as_array().is_none());
    }
}
