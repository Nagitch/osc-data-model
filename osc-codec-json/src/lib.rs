use osc_ir::{IrValue, IrTimestamp, IrBundle, IrBundleElement, IrTimetag};
use serde_json::Value as J;
use base64::Engine;

/// Convert IrBundleElement -> serde_json::Value.
fn bundle_element_to_json(element: &IrBundleElement) -> J {
    match element {
        IrBundleElement::Message(msg) => J::Object([
            ("type".to_string(), J::from("message")),
            ("data".to_string(), to_json(msg)),
        ].into_iter().collect()),
        IrBundleElement::Bundle(bundle) => J::Object([
            ("type".to_string(), J::from("bundle")),
            ("data".to_string(), to_json(&IrValue::Bundle(bundle.clone()))),
        ].into_iter().collect()),
    }
}

/// Convert IR -> serde_json::Value.
pub fn to_json(v: &IrValue) -> J {
    match v {
        IrValue::Null => J::Null,
        IrValue::Bool(b) => J::Bool(*b),
        IrValue::Integer(i) => J::from(*i),
        IrValue::Float(x) => J::from(*x),
        IrValue::String(s) => J::from(s.as_ref()),
        IrValue::Binary(bytes) => J::Object([
            ("$type".to_string(), J::from("binary")),
            ("data".to_string(), J::from(base64::engine::general_purpose::STANDARD.encode(bytes))),
        ].into_iter().collect()),
        IrValue::Array(xs) => J::Array(xs.iter().map(to_json).collect()),
        IrValue::Map(entries) => J::Object(entries.iter().map(|(k, v)| (k.clone(), to_json(v))).collect()),
        IrValue::Timestamp(IrTimestamp{seconds, nanos}) => J::Object([
            ("$type".to_string(), J::from("timestamp")),
            ("seconds".to_string(), J::from(*seconds)),
            ("nanos".to_string(), J::from(*nanos as u64)),
        ].into_iter().collect()),
        IrValue::Ext{ type_id, data } => J::Object([
            ("$type".to_string(), J::from("ext")),
            ("ext".to_string(), J::from(*type_id as i64)),
            ("data".to_string(), J::from(base64::engine::general_purpose::STANDARD.encode(data))),
        ].into_iter().collect()),
        IrValue::Bundle(bundle) => J::Object([
            ("$type".to_string(), J::from("bundle")),
            ("timetag".to_string(), J::from(bundle.timetag.value)),
            ("elements".to_string(), J::Array(bundle.elements.iter().map(bundle_element_to_json).collect())),
        ].into_iter().collect()),
    }
}

/// Convert serde_json::Value -> IrBundleElement.
fn bundle_element_from_json(j: &J) -> IrBundleElement {
    if let J::Object(map) = j {
        if let Some(J::String(element_type)) = map.get("type") {
            match element_type.as_str() {
                "message" => {
                    if let Some(data) = map.get("data") {
                        return IrBundleElement::Message(from_json(data));
                    }
                }
                "bundle" => {
                    if let Some(data) = map.get("data") {
                        if let IrValue::Bundle(bundle) = from_json(data) {
                            return IrBundleElement::Bundle(bundle);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    // Fallback: treat as message
    IrBundleElement::Message(from_json(j))
}

/// Convert serde_json::Value -> IR (best-effort; special objects recognized by $type markers).
pub fn from_json(j: &J) -> IrValue {
    match j {
        J::Null => IrValue::Null,
        J::Bool(b) => IrValue::Bool(*b),
        J::Number(n) => n.as_i64().map(IrValue::Integer)
            .or_else(|| n.as_f64().map(IrValue::Float))
            .unwrap_or(IrValue::Null),
        J::String(s) => IrValue::String(s.clone().into_boxed_str()),
        J::Array(xs) => IrValue::Array(xs.iter().map(from_json).collect()),
        J::Object(map) => {
            if let Some(J::String(tag)) = map.get("$type") {
                match tag.as_str() {
                    "timestamp" => {
                        let sec = map.get("seconds").and_then(|v| v.as_i64()).unwrap_or(0);
                        let ns = map.get("nanos").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                        IrValue::Timestamp(IrTimestamp{ seconds: sec, nanos: ns })
                    }
                    "binary" => {
                        let data = map.get("data").and_then(|v| v.as_str()).map(|s| 
                            base64::engine::general_purpose::STANDARD.decode(s).unwrap_or_default()).unwrap_or_default();
                        IrValue::Binary(data)
                    }
                    "ext" => {
                        let ext = map.get("ext").and_then(|v| v.as_i64()).unwrap_or(0) as i8;
                        let data = map.get("data").and_then(|v| v.as_str()).map(|s| 
                            base64::engine::general_purpose::STANDARD.decode(s).unwrap_or_default()).unwrap_or_default();
                        IrValue::Ext{ type_id: ext, data }
                    }
                    "bundle" => {
                        let timetag_value = map.get("timetag").and_then(|v| v.as_u64()).unwrap_or(1);
                        let timetag = IrTimetag { value: timetag_value };
                        let elements = map.get("elements").and_then(|v| v.as_array())
                            .map(|arr| arr.iter().map(bundle_element_from_json).collect())
                            .unwrap_or_default();
                        IrValue::Bundle(IrBundle { timetag, elements })
                    }
                    _ => IrValue::Map(map.iter().map(|(k,v)| (k.clone(), from_json(v))).collect())
                }
            } else {
                IrValue::Map(map.iter().map(|(k,v)| (k.clone(), from_json(v))).collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use osc_ir::{IrBundle, IrTimetag};

    #[test]
    fn test_bundle_json_roundtrip() {
        // Create a bundle with messages and nested bundle
        let mut bundle = IrBundle::new(IrTimetag::from_ntp(12345));
        bundle.add_message(IrValue::from("hello"));
        bundle.add_message(IrValue::from(42));

        let mut nested_bundle = IrBundle::immediate();
        nested_bundle.add_message(IrValue::from(true));
        nested_bundle.add_message(IrValue::from(3.14));
        
        bundle.add_bundle(nested_bundle);

        let value = IrValue::Bundle(bundle.clone());

        // Convert to JSON and back
        let json = to_json(&value);
        let decoded = from_json(&json);

        assert_eq!(value, decoded);

        // Verify the structure is preserved
        if let IrValue::Bundle(decoded_bundle) = decoded {
            assert_eq!(decoded_bundle.timetag.value, 12345);
            assert_eq!(decoded_bundle.elements.len(), 3);
            
            // Check first message
            assert!(decoded_bundle.elements[0].is_message());
            assert_eq!(
                decoded_bundle.elements[0].as_message().unwrap().as_str(),
                Some("hello")
            );
            
            // Check second message
            assert!(decoded_bundle.elements[1].is_message());
            assert_eq!(
                decoded_bundle.elements[1].as_message().unwrap().as_integer(),
                Some(42)
            );
            
            // Check nested bundle
            assert!(decoded_bundle.elements[2].is_bundle());
            let nested = decoded_bundle.elements[2].as_bundle().unwrap();
            assert!(nested.is_immediate());
            assert_eq!(nested.elements.len(), 2);
        } else {
            panic!("Expected Bundle variant");
        }
    }

    #[test]
    fn test_deeply_nested_bundle_json() {
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
        let json = to_json(&value);
        let decoded = from_json(&json);
        assert_eq!(value, decoded);
    }
}
