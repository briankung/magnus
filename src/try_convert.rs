use crate::{
    debug_assert_value,
    error::Error,
    integer::Integer,
    protect,
    ruby_sys::rb_num2dbl,
    value::{Qnil, Value},
};

pub trait TryConvert<'a>: Sized {
    /// # Safety
    ///
    /// unsafe as typically val must be dereferenced to perform the conversion
    unsafe fn try_convert(val: &'a Value) -> Result<Self, Error>;
}

impl Value {
    pub unsafe fn try_convert<'a, T>(&'a self) -> Result<T, Error>
    where
        T: TryConvert<'a>,
    {
        T::try_convert(self)
    }
}

/// Only implemented on Rust types. TryConvert may convert from a
/// Value to another Ruby type. Use this when you need a Rust value that's
/// divorced from the Ruby runtime, safe to put on the heap, etc.
pub trait TryConvertToRust<'a>: Sized + TryConvert<'a> {
    /// # Safety
    ///
    /// unsafe as typically val must be dereferenced to perform the conversion
    unsafe fn try_convert_to_rust(val: &'a Value) -> Result<Self, Error> {
        Self::try_convert(val)
    }
}

impl TryConvert<'_> for Value {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Ok(*val)
    }
}

impl<'a, T> TryConvert<'a> for Option<T>
where
    T: TryConvert<'a> + 'a,
{
    unsafe fn try_convert(val: &'a Value) -> Result<Self, Error> {
        val.is_nil().then(|| T::try_convert(val)).transpose()
    }
}

impl<'a, T> TryConvertToRust<'a> for Option<T>
where
    T: TryConvertToRust<'a> + 'a,
{
    unsafe fn try_convert_to_rust(val: &'a Value) -> Result<Self, Error> {
        val.is_nil()
            .then(|| T::try_convert_to_rust(val))
            .transpose()
    }
}

impl TryConvert<'_> for bool {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Ok(val.to_bool())
    }
}
impl TryConvertToRust<'_> for bool {}

impl TryConvert<'_> for i8 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_i8()
    }
}
impl TryConvertToRust<'_> for i8 {}

impl TryConvert<'_> for i16 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_i16()
    }
}
impl TryConvertToRust<'_> for i16 {}

impl TryConvert<'_> for i32 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_i32()
    }
}
impl TryConvertToRust<'_> for i32 {}

impl TryConvert<'_> for i64 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_i64()
    }
}
impl TryConvertToRust<'_> for i64 {}

impl TryConvert<'_> for isize {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_isize()
    }
}
impl TryConvertToRust<'_> for isize {}

impl TryConvert<'_> for u8 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_u8()
    }
}
impl TryConvertToRust<'_> for u8 {}

impl TryConvert<'_> for u16 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_u16()
    }
}
impl TryConvertToRust<'_> for u16 {}

impl TryConvert<'_> for u32 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_u32()
    }
}
impl TryConvertToRust<'_> for u32 {}

impl TryConvert<'_> for u64 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_u64()
    }
}
impl TryConvertToRust<'_> for u64 {}

impl TryConvert<'_> for usize {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_usize()
    }
}
impl TryConvertToRust<'_> for usize {}

impl TryConvert<'_> for f32 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        f64::try_convert(val).map(|f| f as f32)
    }
}
impl TryConvertToRust<'_> for f32 {}

impl TryConvert<'_> for f64 {
    unsafe fn try_convert(val: &Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let mut res = 0.0;
        protect(|| {
            res = rb_num2dbl(val.into_inner());
            *Qnil::new()
        })?;
        Ok(res)
    }
}
impl TryConvertToRust<'_> for f64 {}
