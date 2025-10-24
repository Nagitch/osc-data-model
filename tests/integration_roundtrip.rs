#[test]
fn json_roundtrip() {
    use osc_ir::IrValue;
    let v = IrValue::Array(vec![IrValue::Integer(1), IrValue::Bool(true)]);
    let j = osc_codec_json::to_json(&v);
    let v2 = osc_codec_json::from_json(&j);
    assert_eq!(v, v2);
}

#[test]
fn msgpack_roundtrip() {
    use osc_ir::IrValue;
    let v = IrValue::String("ping".into());
    let mp = osc_codec_msgpack::to_msgpack(&v);
    let v2 = osc_codec_msgpack::from_msgpack(&mp);
    assert_eq!(v, v2);
}

#[test]
fn bundle_json_roundtrip() {
    use osc_ir::{IrValue, IrBundle, IrTimetag};
    
    let mut bundle = IrBundle::new(IrTimetag::from_ntp(12345));
    bundle.add_message(IrValue::from("hello world"));
    bundle.add_message(IrValue::from(42));
    
    let mut nested = IrBundle::immediate();
    nested.add_message(IrValue::from(true));
    nested.add_message(IrValue::from(3.14));
    bundle.add_bundle(nested);
    
    let v = IrValue::Bundle(bundle);
    let j = osc_codec_json::to_json(&v);
    let v2 = osc_codec_json::from_json(&j);
    assert_eq!(v, v2);
}

#[test]
fn bundle_msgpack_roundtrip() {
    use osc_ir::{IrValue, IrBundle, IrTimetag};
    
    let mut bundle = IrBundle::new(IrTimetag::from_ntp(54321));
    bundle.add_message(IrValue::from("msgpack test"));
    bundle.add_message(IrValue::from(-123));
    
    let mut nested = IrBundle::immediate();
    nested.add_message(IrValue::from(false));
    nested.add_message(IrValue::from(vec![1_u8, 2, 3, 4]));
    bundle.add_bundle(nested);
    
    let v = IrValue::Bundle(bundle);
    let mp = osc_codec_msgpack::to_msgpack(&v);
    let v2 = osc_codec_msgpack::from_msgpack(&mp);
    assert_eq!(v, v2);
}

#[test]
fn cross_codec_bundle_compatibility() {
    use osc_ir::{IrValue, IrBundle, IrTimetag};
    
    // Create a complex nested bundle
    let mut root = IrBundle::immediate();
    root.add_message(IrValue::from("cross-codec test"));
    
    let mut level1 = IrBundle::new(IrTimetag::from_ntp(1000));
    level1.add_message(IrValue::from(42));
    
    let mut level2 = IrBundle::new(IrTimetag::from_ntp(2000));
    level2.add_message(IrValue::from(true));
    level2.add_message(IrValue::from(vec![0xAA_u8, 0xBB, 0xCC]));
    
    level1.add_bundle(level2);
    root.add_bundle(level1);
    
    let original = IrValue::Bundle(root);
    
    // Test JSON roundtrip
    let json = osc_codec_json::to_json(&original);
    let from_json = osc_codec_json::from_json(&json);
    assert_eq!(original, from_json);
    
    // Test MessagePack roundtrip
    let msgpack = osc_codec_msgpack::to_msgpack(&original);
    let from_msgpack = osc_codec_msgpack::from_msgpack(&msgpack);
    assert_eq!(original, from_msgpack);
    
    // Cross-codec compatibility: both should produce the same result
    assert_eq!(from_json, from_msgpack);
}