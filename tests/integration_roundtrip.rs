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