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
}
