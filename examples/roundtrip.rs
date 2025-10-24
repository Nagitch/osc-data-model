use osc_ir::{IrValue, IrBundle, IrTimetag};

fn main() {
    // Test simple array
    let v = IrValue::Array(vec![IrValue::Integer(42), IrValue::String("ok".into())]);
    let j = osc_codec_json::to_json(&v);
    let v2 = osc_codec_json::from_json(&j);
    let mp = osc_codec_msgpack::to_msgpack(&v);
    let v3 = osc_codec_msgpack::from_msgpack(&mp);
    println!("Array - JSON: {}\nArray - MP: {} bytes\n", j, mp.len());
    assert_eq!(v, v2);
    assert_eq!(v, v3);

    // Test Bundle with nested structure
    let mut bundle = IrBundle::new(IrTimetag::from_ntp(12345));
    bundle.add_message(IrValue::from("Hello Bundle!"));
    bundle.add_message(IrValue::from(123));

    let mut nested_bundle = IrBundle::immediate();
    nested_bundle.add_message(IrValue::from(true));
    nested_bundle.add_message(IrValue::from(vec![0xAA_u8, 0xBB, 0xCC, 0xDD]));
    
    bundle.add_bundle(nested_bundle);

    let bundle_value = IrValue::Bundle(bundle);

    // Test Bundle JSON roundtrip
    let bundle_json = osc_codec_json::to_json(&bundle_value);
    let bundle_v2 = osc_codec_json::from_json(&bundle_json);
    
    // Test Bundle MessagePack roundtrip
    let bundle_mp = osc_codec_msgpack::to_msgpack(&bundle_value);
    let bundle_v3 = osc_codec_msgpack::from_msgpack(&bundle_mp);

    println!("Bundle - JSON: {}\nBundle - MP: {} bytes\n", bundle_json, bundle_mp.len());
    assert_eq!(bundle_value, bundle_v2);
    assert_eq!(bundle_value, bundle_v3);
    
    println!("✓ All roundtrip tests passed!");
    println!("✓ Bundle nesting is working correctly!");
}