use osc_ir::IrValue;

fn main() {
    let v = IrValue::Array(vec![IrValue::Integer(42), IrValue::String("ok".into())]);
    let j = osc_codec_json::to_json(&v);
    let v2 = osc_codec_json::from_json(&j);
    let mp = osc_codec_msgpack::to_msgpack(&v);
    let v3 = osc_codec_msgpack::from_msgpack(&mp);
    println!("JSON: {}\nMP: {} bytes\n", j, mp.len());
    assert_eq!(v, v2);
    assert_eq!(v, v3);
}