use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

use osc_codec_msgpack::try_to_msgpack;
use osc_ir::IrValue;

const MESSAGE_TYPE_TAG: &str = "osc.message";

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OscFfiError {
    Ok = 0,
    NullPointer = 1,
    InvalidUtf8 = 2,
    SerializationError = 3,
    InternalError = 255,
}

#[repr(C)]
pub struct OscMessageHandle {
    address: String,
    args: Vec<IrValue>,
}

impl OscMessageHandle {
    fn new(address: String) -> Self {
        Self {
            address,
            args: Vec::new(),
        }
    }

    fn push_arg(&mut self, value: IrValue) -> OscFfiError {
        if self.args.try_reserve(1).is_err() {
            return OscFfiError::InternalError;
        }
        self.args.push(value);
        OscFfiError::Ok
    }

    fn to_ir_value(&self) -> IrValue {
        let mut map = Vec::with_capacity(3);
        map.push((String::from("$type"), IrValue::from(MESSAGE_TYPE_TAG)));
        map.push((
            String::from("address"),
            IrValue::from(self.address.as_str()),
        ));
        map.push((String::from("args"), IrValue::Array(self.args.clone())));
        IrValue::Map(map)
    }
}

#[repr(C)]
pub struct OscBuffer {
    pub data: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

fn convert_c_str(ptr: *const c_char) -> Result<String, OscFfiError> {
    if ptr.is_null() {
        return Err(OscFfiError::NullPointer);
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map(|s| s.to_owned())
        .map_err(|_| OscFfiError::InvalidUtf8)
}

fn with_message_mut<F>(handle: *mut OscMessageHandle, f: F) -> OscFfiError
where
    F: FnOnce(&mut OscMessageHandle) -> OscFfiError,
{
    if handle.is_null() {
        return OscFfiError::NullPointer;
    }
    let msg = unsafe { &mut *handle };
    f(msg)
}

#[no_mangle]
pub extern "C" fn osc_message_new(
    address: *const c_char,
    out_handle: *mut *mut OscMessageHandle,
) -> OscFfiError {
    if out_handle.is_null() {
        return OscFfiError::NullPointer;
    }
    let addr_str = match convert_c_str(address) {
        Ok(s) => s,
        Err(err) => return err,
    };
    let handle = OscMessageHandle::new(addr_str);
    let boxed = Box::new(handle);
    unsafe {
        *out_handle = Box::into_raw(boxed);
    }
    OscFfiError::Ok
}

#[no_mangle]
pub extern "C" fn osc_message_free(handle: *mut OscMessageHandle) {
    if handle.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub extern "C" fn osc_message_add_i32(handle: *mut OscMessageHandle, value: i32) -> OscFfiError {
    with_message_mut(handle, |msg| msg.push_arg(IrValue::Integer(value as i64)))
}

#[no_mangle]
pub extern "C" fn osc_message_add_f32(handle: *mut OscMessageHandle, value: f32) -> OscFfiError {
    with_message_mut(handle, |msg| msg.push_arg(IrValue::Float(value as f64)))
}

#[no_mangle]
pub extern "C" fn osc_message_add_string(
    handle: *mut OscMessageHandle,
    value: *const c_char,
) -> OscFfiError {
    let string = match convert_c_str(value) {
        Ok(s) => s,
        Err(err) => return err,
    };
    with_message_mut(handle, |msg| msg.push_arg(IrValue::from(string)))
}

#[no_mangle]
pub extern "C" fn osc_message_add_blob(
    handle: *mut OscMessageHandle,
    data: *const u8,
    len: usize,
) -> OscFfiError {
    if len > 0 && data.is_null() {
        return OscFfiError::NullPointer;
    }
    let bytes = if len == 0 {
        Vec::new()
    } else {
        let slice = unsafe { slice::from_raw_parts(data, len) };
        let mut vec = Vec::new();
        if vec.try_reserve(len).is_err() {
            return OscFfiError::InternalError;
        }
        vec.extend_from_slice(slice);
        vec
    };
    with_message_mut(handle, |msg| msg.push_arg(IrValue::from(bytes)))
}

#[no_mangle]
pub extern "C" fn osc_message_to_msgpack(
    handle: *const OscMessageHandle,
    out_buf: *mut *mut OscBuffer,
) -> OscFfiError {
    if handle.is_null() || out_buf.is_null() {
        return OscFfiError::NullPointer;
    }

    let message = unsafe { &*handle };
    let ir_value = message.to_ir_value();

    let bytes = match try_to_msgpack(&ir_value) {
        Ok(data) => data,
        Err(_) => return OscFfiError::SerializationError,
    };

    let mut vec = bytes;
    let buffer = OscBuffer {
        data: vec.as_mut_ptr(),
        len: vec.len(),
        capacity: vec.capacity(),
    };
    std::mem::forget(vec);

    let boxed = Box::new(buffer);
    unsafe {
        *out_buf = Box::into_raw(boxed);
    }

    OscFfiError::Ok
}

#[no_mangle]
pub extern "C" fn osc_buffer_free(buf: *mut OscBuffer) {
    if buf.is_null() {
        return;
    }

    unsafe {
        let buffer = Box::from_raw(buf);
        if !buffer.data.is_null() && buffer.capacity > 0 {
            let _ = Vec::from_raw_parts(buffer.data, buffer.len, buffer.capacity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use osc_codec_msgpack::try_from_msgpack;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn message_roundtrip() {
        let mut handle: *mut OscMessageHandle = ptr::null_mut();
        let address = CString::new("/ffi/test").unwrap();
        assert_eq!(
            OscFfiError::Ok,
            osc_message_new(address.as_ptr(), &mut handle)
        );
        assert!(!handle.is_null());

        assert_eq!(OscFfiError::Ok, osc_message_add_i32(handle, 42));
        assert_eq!(OscFfiError::Ok, osc_message_add_f32(handle, 1.5));

        let text = CString::new("hello").unwrap();
        assert_eq!(
            OscFfiError::Ok,
            osc_message_add_string(handle, text.as_ptr())
        );

        let blob = [1_u8, 2, 3, 4];
        assert_eq!(
            OscFfiError::Ok,
            osc_message_add_blob(handle, blob.as_ptr(), blob.len())
        );

        let mut buffer_ptr: *mut OscBuffer = ptr::null_mut();
        assert_eq!(
            OscFfiError::Ok,
            osc_message_to_msgpack(handle, &mut buffer_ptr)
        );
        assert!(!buffer_ptr.is_null());

        let buffer = unsafe { &*buffer_ptr };
        let bytes = unsafe { slice::from_raw_parts(buffer.data, buffer.len) };
        let value = try_from_msgpack(bytes).expect("decode");
        let map = value.as_map().expect("map");
        assert_eq!(map.len(), 3);

        let args_value = map
            .iter()
            .find(|(k, _)| k == "args")
            .map(|(_, v)| v)
            .expect("args");
        let args = args_value.as_array().expect("array");
        assert_eq!(args.len(), 4);
        assert_eq!(args[0].as_integer(), Some(42));
        assert!((args[1].as_float().unwrap() - 1.5).abs() < f64::EPSILON);
        assert_eq!(args[2].as_str(), Some("hello"));
        assert_eq!(args[3].as_binary(), Some(&blob[..]));

        osc_buffer_free(buffer_ptr);
        osc_message_free(handle);
    }

    #[test]
    fn null_pointer_errors() {
        assert_eq!(
            OscFfiError::NullPointer,
            osc_message_new(ptr::null(), ptr::null_mut())
        );
        let mut handle: *mut OscMessageHandle = ptr::null_mut();
        let address = CString::new("/null").unwrap();
        assert_eq!(
            OscFfiError::Ok,
            osc_message_new(address.as_ptr(), &mut handle)
        );
        assert_eq!(
            OscFfiError::NullPointer,
            osc_message_add_string(handle, ptr::null())
        );
        osc_message_free(handle);
    }
}
