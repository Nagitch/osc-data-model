#![cfg(feature = "osc10")]

use osc_adapter_osc_types as adapter; // crate name with hyphen becomes underscore
use osc_codec_msgpack as msgpack;
use osc_types10 as osc;

#[test]
fn message_ir_msgpack_roundtrip() {
    // Build a basic OSC 1.0 message
    let message = osc::Message {
        address: "/roundtrip",
        args: vec![
            osc::OscType::Int(7),
            osc::OscType::Float(-1.25),
            osc::OscType::String("value"),
            osc::OscType::Blob(&[9, 8, 7]),
        ],
    };

    // OSC -> IrValue
    let ir = adapter::v10::message_to_ir(&message);

    // IrValue -> MsgPack -> IrValue
    let bytes = msgpack::to_msgpack(&ir);
    let ir2 = msgpack::from_msgpack(&bytes);

    // IrValue -> OSC
    let message2 = adapter::v10::ir_to_message(&ir2).expect("expected successful conversion");

    // Validate
    assert_eq!(message2.address, "/roundtrip");
    assert!(matches!(message2.args[0], osc::OscType::Int(7)));
    if let osc::OscType::Float(v) = message2.args[1] {
        assert!((v + 1.25).abs() < f32::EPSILON);
    } else {
        panic!("expected Float arg");
    }
    assert!(matches!(message2.args[2], osc::OscType::String("value")));
    assert!(matches!(message2.args[3], osc::OscType::Blob(slice) if slice == [9, 8, 7]));
}
