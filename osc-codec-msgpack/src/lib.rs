use osc_ir::IrValue;

pub fn to_msgpack(v: &IrValue) -> Vec<u8> {
    // Minimal: reuse serde representation of IrValue
    rmp_serde::to_vec(v).expect("serialize")
}

pub fn from_msgpack(bytes: &[u8]) -> IrValue {
    rmp_serde::from_slice::<IrValue>(bytes).expect("deserialize")
}
