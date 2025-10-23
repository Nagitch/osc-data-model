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
    use osc_ir::IrTimestamp;

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
}
