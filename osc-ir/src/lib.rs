#![cfg_attr(not(test), no_std)]

extern crate alloc;
use alloc::{boxed::Box, string::String, vec::Vec};
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

/// OSC-compatible timetag for bundle scheduling.
/// A value of 1 indicates "immediately", larger values represent NTP-style timestamps.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IrTimetag {
    pub value: u64,
}

impl IrTimetag {
    /// Creates a timetag for immediate execution
    pub fn immediate() -> Self {
        Self { value: 1 }
    }

    /// Creates a timetag from an NTP-style timestamp
    pub fn from_ntp(ntp_time: u64) -> Self {
        Self { value: ntp_time }
    }

    /// Returns true if this timetag indicates immediate execution
    pub fn is_immediate(&self) -> bool {
        self.value == 1
    }
}

/// An element that can be contained within an OSC bundle.
/// Can be either a message (represented as an IrValue) or a nested bundle.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum IrBundleElement {
    /// A message or other data structure
    Message(IrValue),
    /// A nested bundle
    Bundle(IrBundle),
}

/// OSC Bundle structure supporting nested bundles with timetags.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct IrBundle {
    /// When this bundle should be executed
    pub timetag: IrTimetag,
    /// Elements contained in this bundle (messages or nested bundles)
    pub elements: Vec<IrBundleElement>,
}

impl IrBundle {
    /// Creates a new bundle with immediate execution
    pub fn immediate() -> Self {
        Self {
            timetag: IrTimetag::immediate(),
            elements: Vec::new(),
        }
    }

    /// Creates a new bundle with the specified timetag
    pub fn new(timetag: IrTimetag) -> Self {
        Self {
            timetag,
            elements: Vec::new(),
        }
    }

    /// Adds a message to this bundle
    pub fn add_message(&mut self, message: IrValue) {
        self.elements.push(IrBundleElement::Message(message));
    }

    /// Adds a nested bundle to this bundle
    pub fn add_bundle(&mut self, bundle: IrBundle) {
        self.elements.push(IrBundleElement::Bundle(bundle));
    }

    /// Adds an element to this bundle
    pub fn add_element(&mut self, element: IrBundleElement) {
        self.elements.push(element);
    }

    /// Returns true if this bundle is empty (has no elements)
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the number of elements in this bundle
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns true if this bundle should be executed immediately
    pub fn is_immediate(&self) -> bool {
        self.timetag.is_immediate()
    }
}

impl IrBundleElement {
    /// Returns true if this element is a message
    pub fn is_message(&self) -> bool {
        matches!(self, IrBundleElement::Message(_))
    }

    /// Returns true if this element is a bundle
    pub fn is_bundle(&self) -> bool {
        matches!(self, IrBundleElement::Bundle(_))
    }

    /// Returns a reference to the message if this element is a message
    pub fn as_message(&self) -> Option<&IrValue> {
        match self {
            IrBundleElement::Message(msg) => Some(msg),
            _ => None,
        }
    }

    /// Returns a reference to the bundle if this element is a bundle
    pub fn as_bundle(&self) -> Option<&IrBundle> {
        match self {
            IrBundleElement::Bundle(bundle) => Some(bundle),
            _ => None,
        }
    }
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
    /// OSC Bundle with timetag and nested elements
    Bundle(IrBundle),
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

    pub fn as_bundle(&self) -> Option<&IrBundle> {
        match self {
            IrValue::Bundle(bundle) => Some(bundle),
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

impl From<IrBundle> for IrValue {
    fn from(v: IrBundle) -> Self {
        IrValue::Bundle(v)
    }
}

impl From<IrTimetag> for IrBundle {
    fn from(timetag: IrTimetag) -> Self {
        IrBundle {
            timetag,
            elements: Vec::new(),
        }
    }
}

impl From<IrValue> for IrBundleElement {
    fn from(value: IrValue) -> Self {
        IrBundleElement::Message(value)
    }
}

impl From<IrBundle> for IrBundleElement {
    fn from(bundle: IrBundle) -> Self {
        IrBundleElement::Bundle(bundle)
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

    #[test]
    fn bundle_creation_and_nesting() {
        // Create an immediate bundle
        let mut bundle = IrBundle::immediate();
        assert!(bundle.is_immediate());
        assert!(bundle.is_empty());
        assert_eq!(bundle.len(), 0);

        // Add a message
        bundle.add_message(IrValue::from("hello"));
        assert!(!bundle.is_empty());
        assert_eq!(bundle.len(), 1);

        // Create a nested bundle
        let mut nested_bundle = IrBundle::new(IrTimetag::from_ntp(1000));
        assert!(!nested_bundle.is_immediate());
        nested_bundle.add_message(IrValue::from(42));
        nested_bundle.add_message(IrValue::from(true));

        // Add the nested bundle to the main bundle
        bundle.add_bundle(nested_bundle);
        assert_eq!(bundle.len(), 2);

        // Test element access
        assert!(bundle.elements[0].is_message());
        assert!(!bundle.elements[0].is_bundle());
        assert_eq!(bundle.elements[0].as_message().unwrap().as_str(), Some("hello"));

        assert!(!bundle.elements[1].is_message());
        assert!(bundle.elements[1].is_bundle());
        let nested = bundle.elements[1].as_bundle().unwrap();
        assert_eq!(nested.len(), 2);
        assert_eq!(nested.timetag.value, 1000);
    }

    #[test]
    fn bundle_conversions() {
        // Test IrBundle -> IrValue conversion
        let bundle = IrBundle::immediate();
        let value = IrValue::from(bundle.clone());
        assert_eq!(value.as_bundle(), Some(&bundle));

        // Test IrValue -> IrBundleElement conversion
        let message = IrValue::from("test");
        let element = IrBundleElement::from(message.clone());
        assert!(element.is_message());
        assert_eq!(element.as_message(), Some(&message));

        // Test IrBundle -> IrBundleElement conversion
        let element = IrBundleElement::from(bundle.clone());
        assert!(element.is_bundle());
        assert_eq!(element.as_bundle(), Some(&bundle));
    }

    #[test]
    fn timetag_functionality() {
        let immediate = IrTimetag::immediate();
        assert!(immediate.is_immediate());
        assert_eq!(immediate.value, 1);

        let ntp_time = IrTimetag::from_ntp(12345678);
        assert!(!ntp_time.is_immediate());
        assert_eq!(ntp_time.value, 12345678);
    }

    #[test]
    fn complex_nested_bundle_structure() {
        // Create a complex nested structure
        let mut root_bundle = IrBundle::immediate();
        
        // Add some messages
        root_bundle.add_message(IrValue::from("root message 1"));
        root_bundle.add_message(IrValue::from(100));
        
        // Create first nested bundle
        let mut nested1 = IrBundle::new(IrTimetag::from_ntp(2000));
        nested1.add_message(IrValue::from("nested1 message"));
        
        // Create second nested bundle with its own nested bundle
        let mut nested2 = IrBundle::new(IrTimetag::from_ntp(3000));
        nested2.add_message(IrValue::from("nested2 message"));
        
        let mut deeply_nested = IrBundle::new(IrTimetag::from_ntp(4000));
        deeply_nested.add_message(IrValue::from("deeply nested message"));
        deeply_nested.add_message(IrValue::from(3.14));
        
        nested2.add_bundle(deeply_nested);
        
        // Add nested bundles to root
        root_bundle.add_bundle(nested1);
        root_bundle.add_bundle(nested2);
        
        // Verify structure
        assert_eq!(root_bundle.len(), 4); // 2 messages + 2 bundles
        assert!(root_bundle.elements[0].is_message());
        assert!(root_bundle.elements[1].is_message());
        assert!(root_bundle.elements[2].is_bundle());
        assert!(root_bundle.elements[3].is_bundle());
        
        // Check the second nested bundle contains a bundle
        let nested2_ref = root_bundle.elements[3].as_bundle().unwrap();
        assert_eq!(nested2_ref.len(), 2); // 1 message + 1 bundle
        assert!(nested2_ref.elements[1].is_bundle());
        
        // Check deeply nested bundle
        let deeply_nested_ref = nested2_ref.elements[1].as_bundle().unwrap();
        assert_eq!(deeply_nested_ref.len(), 2);
        assert_eq!(deeply_nested_ref.timetag.value, 4000);
    }
}
