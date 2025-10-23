use osc_ir::{IrValue, IrTimestamp};
use serde_json::Value as J;

/// Convert IR -> serde_json::Value.
pub fn to_json(v: &IrValue) -> J {
    match v {
        IrValue::Null => J::Null,
        IrValue::Bool(b) => J::Bool(*b),
        IrValue::Integer(i) => J::from(*i),
        IrValue::Float(x) => J::from(*x),
        IrValue::String(s) => J::from(s.as_ref()),
        IrValue::Binary(bytes) => J::from(base64::engine::general_purpose::STANDARD.encode(bytes)),
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
    }
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
                    "ext" => {
                        let ext = map.get("ext").and_then(|v| v.as_i64()).unwrap_or(0) as i8;
                        let data = map.get("data").and_then(|v| v.as_str()).map(|s| 
                            base64::engine::general_purpose::STANDARD.decode(s).unwrap_or_default()).unwrap_or_default();
                        IrValue::Ext{ type_id: ext, data }
                    }
                    _ => IrValue::Map(map.iter().map(|(k,v)| (k.clone(), from_json(v))).collect())
                }
            } else {
                IrValue::Map(map.iter().map(|(k,v)| (k.clone(), from_json(v))).collect())
            }
        }
    }
}
