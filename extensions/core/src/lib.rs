use std::os::raw::{c_char, c_void};
pub type ResultCode = i32;
pub use limbo_macros::export_scalar;
pub const RESULT_OK: ResultCode = 0;
pub const RESULT_ERROR: ResultCode = 1;
// TODO: more error types

pub type ExtensionEntryPoint = extern "C" fn(api: *const ExtensionApi) -> ResultCode;
pub type ScalarFunction = extern "C" fn(argc: i32, *const Value) -> Value;

#[repr(C)]
pub struct ExtensionApi {
    pub ctx: *mut c_void,
    pub register_scalar_function:
        extern "C" fn(ctx: *mut c_void, name: *const c_char, func: ScalarFunction) -> ResultCode,
}

#[macro_export]
macro_rules! register_extension {
    (
        scalars: { $( $scalar_name:expr => $scalar_func:ident ),* $(,)? },
        //aggregates: { $( $agg_name:expr => ($step_func:ident, $finalize_func:ident) ),* $(,)? },
        //virtual_tables: { $( $vt_name:expr => $vt_impl:expr ),* $(,)? }
    ) => {
        #[no_mangle]
        pub unsafe extern "C" fn register_extension(api: *const $crate::ExtensionApi) -> $crate::ResultCode {
            if api.is_null() {
                return $crate::RESULT_ERROR;
            }

            register_scalar_functions! { api, $( $scalar_name => $scalar_func ),* }
            // TODO:
            //register_aggregate_functions! { $( $agg_name => ($step_func, $finalize_func) ),* }
            //register_virtual_tables! { $( $vt_name => $vt_impl ),* }
            $crate::RESULT_OK
        }
    }
}

#[macro_export]
macro_rules! register_scalar_functions {
    ( $api:expr, $( $fname:expr => $fptr:ident ),* ) => {
        unsafe {
            $(
                let cname = std::ffi::CString::new($fname).unwrap();
                ((*$api).register_scalar_function)((*$api).ctx, cname.as_ptr(), $fptr);
            )*
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ValueType {
    Null,
    Integer,
    Float,
    Text,
    Blob,
}

#[repr(C)]
pub struct Value {
    value_type: ValueType,
    value: *mut c_void,
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value_type {
            ValueType::Null => write!(f, "Value {{ Null }}"),
            ValueType::Integer => write!(f, "Value {{ Integer: {} }}", unsafe {
                *(self.value as *const i64)
            }),
            ValueType::Float => write!(f, "Value {{ Float: {} }}", unsafe {
                *(self.value as *const f64)
            }),
            ValueType::Text => write!(f, "Value {{ Text: {:?} }}", unsafe {
                &*(self.value as *const TextValue)
            }),
            ValueType::Blob => write!(f, "Value {{ Blob: {:?} }}", unsafe {
                &*(self.value as *const Blob)
            }),
        }
    }
}

#[repr(C)]
pub struct TextValue {
    text: *const u8,
    len: u32,
}

impl std::fmt::Debug for TextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TextValue {{ text: {:?}, len: {} }}",
            self.text, self.len
        )
    }
}

impl Default for TextValue {
    fn default() -> Self {
        Self {
            text: std::ptr::null(),
            len: 0,
        }
    }
}

impl TextValue {
    pub(crate) fn new(text: *const u8, len: usize) -> Self {
        Self {
            text,
            len: len as u32,
        }
    }

    fn as_str(&self) -> &str {
        if self.text.is_null() {
            return "";
        }
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.text, self.len as usize))
        }
    }
}

#[repr(C)]
pub struct Blob {
    data: *const u8,
    size: u64,
}

impl std::fmt::Debug for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blob {{ data: {:?}, size: {} }}", self.data, self.size)
    }
}

impl Blob {
    pub fn new(data: *const u8, size: u64) -> Self {
        Self { data, size }
    }
}

impl Value {
    pub fn null() -> Self {
        Self {
            value_type: ValueType::Null,
            value: std::ptr::null_mut(),
        }
    }

    pub fn value_type(&self) -> ValueType {
        self.value_type
    }

    pub fn to_float(&self) -> Option<f64> {
        if self.value_type != ValueType::Float {
            return None;
        }
        if self.value.is_null() {
            return None;
        }
        Some(unsafe { *(self.value as *const f64) })
    }

    pub fn to_text(&self) -> Option<String> {
        if self.value_type != ValueType::Text {
            return None;
        }
        if self.value.is_null() {
            return None;
        }
        let txt = unsafe { &*(self.value as *const TextValue) };
        Some(String::from(txt.as_str()))
    }

    pub fn to_blob(&self) -> Option<Vec<u8>> {
        if self.value_type != ValueType::Blob {
            return None;
        }
        if self.value.is_null() {
            return None;
        }
        let blob = unsafe { &*(self.value as *const Blob) };
        let slice = unsafe { std::slice::from_raw_parts(blob.data, blob.size as usize) };
        Some(slice.to_vec())
    }

    pub fn to_integer(&self) -> Option<i64> {
        if self.value_type != ValueType::Integer {
            return None;
        }
        if self.value.is_null() {
            return None;
        }
        Some(unsafe { *(self.value as *const i64) })
    }

    pub fn from_integer(value: i64) -> Self {
        let boxed = Box::new(value);
        Self {
            value_type: ValueType::Integer,
            value: Box::into_raw(boxed) as *mut c_void,
        }
    }

    pub fn from_float(value: f64) -> Self {
        let boxed = Box::new(value);
        Self {
            value_type: ValueType::Float,
            value: Box::into_raw(boxed) as *mut c_void,
        }
    }

    pub fn from_text(s: String) -> Self {
        let buffer = s.into_boxed_str();
        let ptr = buffer.as_ptr();
        let len = buffer.len();
        std::mem::forget(buffer);
        let text_value = TextValue::new(ptr, len);
        let text_box = Box::new(text_value);
        Self {
            value_type: ValueType::Text,
            value: Box::into_raw(text_box) as *mut c_void,
        }
    }

    pub fn from_blob(value: Vec<u8>) -> Self {
        let boxed = Box::new(Blob::new(value.as_ptr(), value.len() as u64));
        std::mem::forget(value);
        Self {
            value_type: ValueType::Blob,
            value: Box::into_raw(boxed) as *mut c_void,
        }
    }

    /// # Safety
    /// consumes the value while freeing the underlying memory with null check.
    /// however this does assume that the type was properly constructed with
    /// the appropriate value_type and value.
    pub unsafe fn free(self) {
        if self.value.is_null() {
            return;
        }
        match self.value_type {
            ValueType::Integer => {
                let _ = Box::from_raw(self.value as *mut i64);
            }
            ValueType::Float => {
                let _ = Box::from_raw(self.value as *mut f64);
            }
            ValueType::Text => {
                let _ = Box::from_raw(self.value as *mut TextValue);
            }
            ValueType::Blob => {
                let _ = Box::from_raw(self.value as *mut Blob);
            }
            ValueType::Null => {}
        }
    }
}
