#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::{string::String, vec::Vec, boxed::Box};
use core::fmt;

#[cfg(feature = "serde")] use serde::{Serialize, Deserialize};

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
    Ext { type_id: i8, #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] data: Vec<u8> },
}

impl fmt::Display for IrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

impl IrValue {
    pub fn null() -> Self { IrValue::Null }
}
