use osc_ir::IrValue;

#[cfg(feature = "osc10")]
pub mod v10 {
    use super::*;
    use osc_types10 as osc;

    pub fn message_to_ir(_m: &osc::OscMessage) -> IrValue {
        // TODO: map OSC 1.0 message to IrValue (path, args, etc.)
        IrValue::Null
    }

    pub fn ir_to_message(_v: &IrValue) -> Option<osc::OscMessage> {
        // TODO: reconstruct message if representable
        None
    }
}

#[cfg(feature = "osc11")]
pub mod v11 {
    use super::*;
    use osc_types11 as osc;
    // TODO: same as v10 for OSC 1.1
}
