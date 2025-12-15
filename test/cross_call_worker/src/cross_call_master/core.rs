// Generated from cross_call_master.pplugin (group: core)

#[allow(unused_imports)]
use std::sync::RwLock;
#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use super::delegates::*;
#[allow(unused_imports)]
use plugify::{get_method_ptr, Str, Arr, Var, Vec2, Vec3, Vec4, Mat4x4};

/// # Arguments
/// * `returnString` - (string)
#[allow(dead_code, non_snake_case)]
pub fn ReverseReturn(returnString: &Str) {
    unsafe { __cross_call_master_ReverseReturn.expect("ReverseReturn function was not found")(returnString) }
}
pub type _ReverseReturn = unsafe extern "C" fn(&Str);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ReverseReturn: Option<_ReverseReturn> = None;

#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnVoidCallback() {
    unsafe { __cross_call_master_NoParamReturnVoidCallback.expect("NoParamReturnVoidCallback function was not found")() }
}
pub type _NoParamReturnVoidCallback = unsafe extern "C" fn();
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnVoidCallback: Option<_NoParamReturnVoidCallback> = None;

///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnBoolCallback() -> bool {
    unsafe { __cross_call_master_NoParamReturnBoolCallback.expect("NoParamReturnBoolCallback function was not found")() }
}
pub type _NoParamReturnBoolCallback = unsafe extern "C" fn() -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnBoolCallback: Option<_NoParamReturnBoolCallback> = None;

///
/// # Returns
/// char8
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnChar8Callback() -> i8 {
    unsafe { __cross_call_master_NoParamReturnChar8Callback.expect("NoParamReturnChar8Callback function was not found")() }
}
pub type _NoParamReturnChar8Callback = unsafe extern "C" fn() -> i8;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnChar8Callback: Option<_NoParamReturnChar8Callback> = None;

///
/// # Returns
/// char16
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnChar16Callback() -> u16 {
    unsafe { __cross_call_master_NoParamReturnChar16Callback.expect("NoParamReturnChar16Callback function was not found")() }
}
pub type _NoParamReturnChar16Callback = unsafe extern "C" fn() -> u16;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnChar16Callback: Option<_NoParamReturnChar16Callback> = None;

///
/// # Returns
/// int8
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnInt8Callback() -> i8 {
    unsafe { __cross_call_master_NoParamReturnInt8Callback.expect("NoParamReturnInt8Callback function was not found")() }
}
pub type _NoParamReturnInt8Callback = unsafe extern "C" fn() -> i8;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnInt8Callback: Option<_NoParamReturnInt8Callback> = None;

///
/// # Returns
/// int16
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnInt16Callback() -> i16 {
    unsafe { __cross_call_master_NoParamReturnInt16Callback.expect("NoParamReturnInt16Callback function was not found")() }
}
pub type _NoParamReturnInt16Callback = unsafe extern "C" fn() -> i16;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnInt16Callback: Option<_NoParamReturnInt16Callback> = None;

///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnInt32Callback() -> i32 {
    unsafe { __cross_call_master_NoParamReturnInt32Callback.expect("NoParamReturnInt32Callback function was not found")() }
}
pub type _NoParamReturnInt32Callback = unsafe extern "C" fn() -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnInt32Callback: Option<_NoParamReturnInt32Callback> = None;

///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnInt64Callback() -> i64 {
    unsafe { __cross_call_master_NoParamReturnInt64Callback.expect("NoParamReturnInt64Callback function was not found")() }
}
pub type _NoParamReturnInt64Callback = unsafe extern "C" fn() -> i64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnInt64Callback: Option<_NoParamReturnInt64Callback> = None;

///
/// # Returns
/// uint8
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnUInt8Callback() -> u8 {
    unsafe { __cross_call_master_NoParamReturnUInt8Callback.expect("NoParamReturnUInt8Callback function was not found")() }
}
pub type _NoParamReturnUInt8Callback = unsafe extern "C" fn() -> u8;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnUInt8Callback: Option<_NoParamReturnUInt8Callback> = None;

///
/// # Returns
/// uint16
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnUInt16Callback() -> u16 {
    unsafe { __cross_call_master_NoParamReturnUInt16Callback.expect("NoParamReturnUInt16Callback function was not found")() }
}
pub type _NoParamReturnUInt16Callback = unsafe extern "C" fn() -> u16;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnUInt16Callback: Option<_NoParamReturnUInt16Callback> = None;

///
/// # Returns
/// uint32
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnUInt32Callback() -> u32 {
    unsafe { __cross_call_master_NoParamReturnUInt32Callback.expect("NoParamReturnUInt32Callback function was not found")() }
}
pub type _NoParamReturnUInt32Callback = unsafe extern "C" fn() -> u32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnUInt32Callback: Option<_NoParamReturnUInt32Callback> = None;

///
/// # Returns
/// uint64
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnUInt64Callback() -> u64 {
    unsafe { __cross_call_master_NoParamReturnUInt64Callback.expect("NoParamReturnUInt64Callback function was not found")() }
}
pub type _NoParamReturnUInt64Callback = unsafe extern "C" fn() -> u64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnUInt64Callback: Option<_NoParamReturnUInt64Callback> = None;

///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnPointerCallback() -> usize {
    unsafe { __cross_call_master_NoParamReturnPointerCallback.expect("NoParamReturnPointerCallback function was not found")() }
}
pub type _NoParamReturnPointerCallback = unsafe extern "C" fn() -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnPointerCallback: Option<_NoParamReturnPointerCallback> = None;

///
/// # Returns
/// float
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnFloatCallback() -> f32 {
    unsafe { __cross_call_master_NoParamReturnFloatCallback.expect("NoParamReturnFloatCallback function was not found")() }
}
pub type _NoParamReturnFloatCallback = unsafe extern "C" fn() -> f32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnFloatCallback: Option<_NoParamReturnFloatCallback> = None;

///
/// # Returns
/// double
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnDoubleCallback() -> f64 {
    unsafe { __cross_call_master_NoParamReturnDoubleCallback.expect("NoParamReturnDoubleCallback function was not found")() }
}
pub type _NoParamReturnDoubleCallback = unsafe extern "C" fn() -> f64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnDoubleCallback: Option<_NoParamReturnDoubleCallback> = None;

///
/// # Returns
/// function
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnFunctionCallback() -> NoParamReturnFunctionCallbackFunc {
    unsafe { __cross_call_master_NoParamReturnFunctionCallback.expect("NoParamReturnFunctionCallback function was not found")() }
}
pub type _NoParamReturnFunctionCallback = unsafe extern "C" fn() -> NoParamReturnFunctionCallbackFunc;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnFunctionCallback: Option<_NoParamReturnFunctionCallback> = None;

///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnStringCallback() -> Str {
    unsafe { __cross_call_master_NoParamReturnStringCallback.expect("NoParamReturnStringCallback function was not found")() }
}
pub type _NoParamReturnStringCallback = unsafe extern "C" fn() -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnStringCallback: Option<_NoParamReturnStringCallback> = None;

///
/// # Returns
/// any
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnAnyCallback() -> Var {
    unsafe { __cross_call_master_NoParamReturnAnyCallback.expect("NoParamReturnAnyCallback function was not found")() }
}
pub type _NoParamReturnAnyCallback = unsafe extern "C" fn() -> Var;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnAnyCallback: Option<_NoParamReturnAnyCallback> = None;

///
/// # Returns
/// bool[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayBoolCallback() -> Arr<bool> {
    unsafe { __cross_call_master_NoParamReturnArrayBoolCallback.expect("NoParamReturnArrayBoolCallback function was not found")() }
}
pub type _NoParamReturnArrayBoolCallback = unsafe extern "C" fn() -> Arr<bool>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayBoolCallback: Option<_NoParamReturnArrayBoolCallback> = None;

///
/// # Returns
/// char8[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayChar8Callback() -> Arr<i8> {
    unsafe { __cross_call_master_NoParamReturnArrayChar8Callback.expect("NoParamReturnArrayChar8Callback function was not found")() }
}
pub type _NoParamReturnArrayChar8Callback = unsafe extern "C" fn() -> Arr<i8>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayChar8Callback: Option<_NoParamReturnArrayChar8Callback> = None;

///
/// # Returns
/// char16[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayChar16Callback() -> Arr<u16> {
    unsafe { __cross_call_master_NoParamReturnArrayChar16Callback.expect("NoParamReturnArrayChar16Callback function was not found")() }
}
pub type _NoParamReturnArrayChar16Callback = unsafe extern "C" fn() -> Arr<u16>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayChar16Callback: Option<_NoParamReturnArrayChar16Callback> = None;

///
/// # Returns
/// int8[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayInt8Callback() -> Arr<i8> {
    unsafe { __cross_call_master_NoParamReturnArrayInt8Callback.expect("NoParamReturnArrayInt8Callback function was not found")() }
}
pub type _NoParamReturnArrayInt8Callback = unsafe extern "C" fn() -> Arr<i8>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayInt8Callback: Option<_NoParamReturnArrayInt8Callback> = None;

///
/// # Returns
/// int16[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayInt16Callback() -> Arr<i16> {
    unsafe { __cross_call_master_NoParamReturnArrayInt16Callback.expect("NoParamReturnArrayInt16Callback function was not found")() }
}
pub type _NoParamReturnArrayInt16Callback = unsafe extern "C" fn() -> Arr<i16>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayInt16Callback: Option<_NoParamReturnArrayInt16Callback> = None;

///
/// # Returns
/// int32[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayInt32Callback() -> Arr<i32> {
    unsafe { __cross_call_master_NoParamReturnArrayInt32Callback.expect("NoParamReturnArrayInt32Callback function was not found")() }
}
pub type _NoParamReturnArrayInt32Callback = unsafe extern "C" fn() -> Arr<i32>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayInt32Callback: Option<_NoParamReturnArrayInt32Callback> = None;

///
/// # Returns
/// int64[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayInt64Callback() -> Arr<i64> {
    unsafe { __cross_call_master_NoParamReturnArrayInt64Callback.expect("NoParamReturnArrayInt64Callback function was not found")() }
}
pub type _NoParamReturnArrayInt64Callback = unsafe extern "C" fn() -> Arr<i64>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayInt64Callback: Option<_NoParamReturnArrayInt64Callback> = None;

///
/// # Returns
/// uint8[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayUInt8Callback() -> Arr<u8> {
    unsafe { __cross_call_master_NoParamReturnArrayUInt8Callback.expect("NoParamReturnArrayUInt8Callback function was not found")() }
}
pub type _NoParamReturnArrayUInt8Callback = unsafe extern "C" fn() -> Arr<u8>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayUInt8Callback: Option<_NoParamReturnArrayUInt8Callback> = None;

///
/// # Returns
/// uint16[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayUInt16Callback() -> Arr<u16> {
    unsafe { __cross_call_master_NoParamReturnArrayUInt16Callback.expect("NoParamReturnArrayUInt16Callback function was not found")() }
}
pub type _NoParamReturnArrayUInt16Callback = unsafe extern "C" fn() -> Arr<u16>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayUInt16Callback: Option<_NoParamReturnArrayUInt16Callback> = None;

///
/// # Returns
/// uint32[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayUInt32Callback() -> Arr<u32> {
    unsafe { __cross_call_master_NoParamReturnArrayUInt32Callback.expect("NoParamReturnArrayUInt32Callback function was not found")() }
}
pub type _NoParamReturnArrayUInt32Callback = unsafe extern "C" fn() -> Arr<u32>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayUInt32Callback: Option<_NoParamReturnArrayUInt32Callback> = None;

///
/// # Returns
/// uint64[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayUInt64Callback() -> Arr<u64> {
    unsafe { __cross_call_master_NoParamReturnArrayUInt64Callback.expect("NoParamReturnArrayUInt64Callback function was not found")() }
}
pub type _NoParamReturnArrayUInt64Callback = unsafe extern "C" fn() -> Arr<u64>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayUInt64Callback: Option<_NoParamReturnArrayUInt64Callback> = None;

///
/// # Returns
/// ptr64[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayPointerCallback() -> Arr<usize> {
    unsafe { __cross_call_master_NoParamReturnArrayPointerCallback.expect("NoParamReturnArrayPointerCallback function was not found")() }
}
pub type _NoParamReturnArrayPointerCallback = unsafe extern "C" fn() -> Arr<usize>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayPointerCallback: Option<_NoParamReturnArrayPointerCallback> = None;

///
/// # Returns
/// float[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayFloatCallback() -> Arr<f32> {
    unsafe { __cross_call_master_NoParamReturnArrayFloatCallback.expect("NoParamReturnArrayFloatCallback function was not found")() }
}
pub type _NoParamReturnArrayFloatCallback = unsafe extern "C" fn() -> Arr<f32>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayFloatCallback: Option<_NoParamReturnArrayFloatCallback> = None;

///
/// # Returns
/// double[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayDoubleCallback() -> Arr<f64> {
    unsafe { __cross_call_master_NoParamReturnArrayDoubleCallback.expect("NoParamReturnArrayDoubleCallback function was not found")() }
}
pub type _NoParamReturnArrayDoubleCallback = unsafe extern "C" fn() -> Arr<f64>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayDoubleCallback: Option<_NoParamReturnArrayDoubleCallback> = None;

///
/// # Returns
/// string[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayStringCallback() -> Arr<Str> {
    unsafe { __cross_call_master_NoParamReturnArrayStringCallback.expect("NoParamReturnArrayStringCallback function was not found")() }
}
pub type _NoParamReturnArrayStringCallback = unsafe extern "C" fn() -> Arr<Str>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayStringCallback: Option<_NoParamReturnArrayStringCallback> = None;

///
/// # Returns
/// any[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayAnyCallback() -> Arr<Var> {
    unsafe { __cross_call_master_NoParamReturnArrayAnyCallback.expect("NoParamReturnArrayAnyCallback function was not found")() }
}
pub type _NoParamReturnArrayAnyCallback = unsafe extern "C" fn() -> Arr<Var>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayAnyCallback: Option<_NoParamReturnArrayAnyCallback> = None;

///
/// # Returns
/// vec2[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayVector2Callback() -> Arr<Vec2> {
    unsafe { __cross_call_master_NoParamReturnArrayVector2Callback.expect("NoParamReturnArrayVector2Callback function was not found")() }
}
pub type _NoParamReturnArrayVector2Callback = unsafe extern "C" fn() -> Arr<Vec2>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayVector2Callback: Option<_NoParamReturnArrayVector2Callback> = None;

///
/// # Returns
/// vec3[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayVector3Callback() -> Arr<Vec3> {
    unsafe { __cross_call_master_NoParamReturnArrayVector3Callback.expect("NoParamReturnArrayVector3Callback function was not found")() }
}
pub type _NoParamReturnArrayVector3Callback = unsafe extern "C" fn() -> Arr<Vec3>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayVector3Callback: Option<_NoParamReturnArrayVector3Callback> = None;

///
/// # Returns
/// vec4[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayVector4Callback() -> Arr<Vec4> {
    unsafe { __cross_call_master_NoParamReturnArrayVector4Callback.expect("NoParamReturnArrayVector4Callback function was not found")() }
}
pub type _NoParamReturnArrayVector4Callback = unsafe extern "C" fn() -> Arr<Vec4>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayVector4Callback: Option<_NoParamReturnArrayVector4Callback> = None;

///
/// # Returns
/// mat4x4[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayMatrix4x4Callback() -> Arr<Mat4x4> {
    unsafe { __cross_call_master_NoParamReturnArrayMatrix4x4Callback.expect("NoParamReturnArrayMatrix4x4Callback function was not found")() }
}
pub type _NoParamReturnArrayMatrix4x4Callback = unsafe extern "C" fn() -> Arr<Mat4x4>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnArrayMatrix4x4Callback: Option<_NoParamReturnArrayMatrix4x4Callback> = None;

///
/// # Returns
/// vec2
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnVector2Callback() -> Vec2 {
    unsafe { __cross_call_master_NoParamReturnVector2Callback.expect("NoParamReturnVector2Callback function was not found")() }
}
pub type _NoParamReturnVector2Callback = unsafe extern "C" fn() -> Vec2;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnVector2Callback: Option<_NoParamReturnVector2Callback> = None;

///
/// # Returns
/// vec3
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnVector3Callback() -> Vec3 {
    unsafe { __cross_call_master_NoParamReturnVector3Callback.expect("NoParamReturnVector3Callback function was not found")() }
}
pub type _NoParamReturnVector3Callback = unsafe extern "C" fn() -> Vec3;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnVector3Callback: Option<_NoParamReturnVector3Callback> = None;

///
/// # Returns
/// vec4
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnVector4Callback() -> Vec4 {
    unsafe { __cross_call_master_NoParamReturnVector4Callback.expect("NoParamReturnVector4Callback function was not found")() }
}
pub type _NoParamReturnVector4Callback = unsafe extern "C" fn() -> Vec4;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnVector4Callback: Option<_NoParamReturnVector4Callback> = None;

///
/// # Returns
/// mat4x4
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnMatrix4x4Callback() -> Mat4x4 {
    unsafe { __cross_call_master_NoParamReturnMatrix4x4Callback.expect("NoParamReturnMatrix4x4Callback function was not found")() }
}
pub type _NoParamReturnMatrix4x4Callback = unsafe extern "C" fn() -> Mat4x4;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_NoParamReturnMatrix4x4Callback: Option<_NoParamReturnMatrix4x4Callback> = None;

/// # Arguments
/// * `a` - (int32)
#[allow(dead_code, non_snake_case)]
pub fn Param1Callback(a: i32) {
    unsafe { __cross_call_master_Param1Callback.expect("Param1Callback function was not found")(a) }
}
pub type _Param1Callback = unsafe extern "C" fn(i32);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param1Callback: Option<_Param1Callback> = None;

/// # Arguments
/// * `a` - (int32)
/// * `b` - (float)
#[allow(dead_code, non_snake_case)]
pub fn Param2Callback(a: i32, b: f32) {
    unsafe { __cross_call_master_Param2Callback.expect("Param2Callback function was not found")(a, b) }
}
pub type _Param2Callback = unsafe extern "C" fn(i32, f32);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param2Callback: Option<_Param2Callback> = None;

/// # Arguments
/// * `a` - (int32)
/// * `b` - (float)
/// * `c` - (double)
#[allow(dead_code, non_snake_case)]
pub fn Param3Callback(a: i32, b: f32, c: f64) {
    unsafe { __cross_call_master_Param3Callback.expect("Param3Callback function was not found")(a, b, c) }
}
pub type _Param3Callback = unsafe extern "C" fn(i32, f32, f64);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param3Callback: Option<_Param3Callback> = None;

/// # Arguments
/// * `a` - (int32)
/// * `b` - (float)
/// * `c` - (double)
/// * `d` - (vec4)
#[allow(dead_code, non_snake_case)]
pub fn Param4Callback(a: i32, b: f32, c: f64, d: &Vec4) {
    unsafe { __cross_call_master_Param4Callback.expect("Param4Callback function was not found")(a, b, c, d) }
}
pub type _Param4Callback = unsafe extern "C" fn(i32, f32, f64, &Vec4);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param4Callback: Option<_Param4Callback> = None;

/// # Arguments
/// * `a` - (int32)
/// * `b` - (float)
/// * `c` - (double)
/// * `d` - (vec4)
/// * `e` - (int64[])
#[allow(dead_code, non_snake_case)]
pub fn Param5Callback(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>) {
    unsafe { __cross_call_master_Param5Callback.expect("Param5Callback function was not found")(a, b, c, d, e) }
}
pub type _Param5Callback = unsafe extern "C" fn(i32, f32, f64, &Vec4, &Arr<i64>);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param5Callback: Option<_Param5Callback> = None;

/// # Arguments
/// * `a` - (int32)
/// * `b` - (float)
/// * `c` - (double)
/// * `d` - (vec4)
/// * `e` - (int64[])
/// * `f` - (char8)
#[allow(dead_code, non_snake_case)]
pub fn Param6Callback(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8) {
    unsafe { __cross_call_master_Param6Callback.expect("Param6Callback function was not found")(a, b, c, d, e, f) }
}
pub type _Param6Callback = unsafe extern "C" fn(i32, f32, f64, &Vec4, &Arr<i64>, i8);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param6Callback: Option<_Param6Callback> = None;

/// # Arguments
/// * `a` - (int32)
/// * `b` - (float)
/// * `c` - (double)
/// * `d` - (vec4)
/// * `e` - (int64[])
/// * `f` - (char8)
/// * `g` - (string)
#[allow(dead_code, non_snake_case)]
pub fn Param7Callback(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8, g: &Str) {
    unsafe { __cross_call_master_Param7Callback.expect("Param7Callback function was not found")(a, b, c, d, e, f, g) }
}
pub type _Param7Callback = unsafe extern "C" fn(i32, f32, f64, &Vec4, &Arr<i64>, i8, &Str);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param7Callback: Option<_Param7Callback> = None;

/// # Arguments
/// * `a` - (int32)
/// * `b` - (float)
/// * `c` - (double)
/// * `d` - (vec4)
/// * `e` - (int64[])
/// * `f` - (char8)
/// * `g` - (string)
/// * `h` - (char16)
#[allow(dead_code, non_snake_case)]
pub fn Param8Callback(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8, g: &Str, h: u16) {
    unsafe { __cross_call_master_Param8Callback.expect("Param8Callback function was not found")(a, b, c, d, e, f, g, h) }
}
pub type _Param8Callback = unsafe extern "C" fn(i32, f32, f64, &Vec4, &Arr<i64>, i8, &Str, u16);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param8Callback: Option<_Param8Callback> = None;

/// # Arguments
/// * `a` - (int32)
/// * `b` - (float)
/// * `c` - (double)
/// * `d` - (vec4)
/// * `e` - (int64[])
/// * `f` - (char8)
/// * `g` - (string)
/// * `h` - (char16)
/// * `k` - (int16)
#[allow(dead_code, non_snake_case)]
pub fn Param9Callback(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8, g: &Str, h: u16, k: i16) {
    unsafe { __cross_call_master_Param9Callback.expect("Param9Callback function was not found")(a, b, c, d, e, f, g, h, k) }
}
pub type _Param9Callback = unsafe extern "C" fn(i32, f32, f64, &Vec4, &Arr<i64>, i8, &Str, u16, i16);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param9Callback: Option<_Param9Callback> = None;

/// # Arguments
/// * `a` - (int32)
/// * `b` - (float)
/// * `c` - (double)
/// * `d` - (vec4)
/// * `e` - (int64[])
/// * `f` - (char8)
/// * `g` - (string)
/// * `h` - (char16)
/// * `k` - (int16)
/// * `l` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn Param10Callback(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8, g: &Str, h: u16, k: i16, l: usize) {
    unsafe { __cross_call_master_Param10Callback.expect("Param10Callback function was not found")(a, b, c, d, e, f, g, h, k, l) }
}
pub type _Param10Callback = unsafe extern "C" fn(i32, f32, f64, &Vec4, &Arr<i64>, i8, &Str, u16, i16, usize);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_Param10Callback: Option<_Param10Callback> = None;

/// # Arguments
/// * `a` - (int32&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef1Callback(a: &mut i32) {
    unsafe { __cross_call_master_ParamRef1Callback.expect("ParamRef1Callback function was not found")(a) }
}
pub type _ParamRef1Callback = unsafe extern "C" fn(&mut i32);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef1Callback: Option<_ParamRef1Callback> = None;

/// # Arguments
/// * `a` - (int32&)
/// * `b` - (float&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef2Callback(a: &mut i32, b: &mut f32) {
    unsafe { __cross_call_master_ParamRef2Callback.expect("ParamRef2Callback function was not found")(a, b) }
}
pub type _ParamRef2Callback = unsafe extern "C" fn(&mut i32, &mut f32);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef2Callback: Option<_ParamRef2Callback> = None;

/// # Arguments
/// * `a` - (int32&)
/// * `b` - (float&)
/// * `c` - (double&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef3Callback(a: &mut i32, b: &mut f32, c: &mut f64) {
    unsafe { __cross_call_master_ParamRef3Callback.expect("ParamRef3Callback function was not found")(a, b, c) }
}
pub type _ParamRef3Callback = unsafe extern "C" fn(&mut i32, &mut f32, &mut f64);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef3Callback: Option<_ParamRef3Callback> = None;

/// # Arguments
/// * `a` - (int32&)
/// * `b` - (float&)
/// * `c` - (double&)
/// * `d` - (vec4&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef4Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4) {
    unsafe { __cross_call_master_ParamRef4Callback.expect("ParamRef4Callback function was not found")(a, b, c, d) }
}
pub type _ParamRef4Callback = unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vec4);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef4Callback: Option<_ParamRef4Callback> = None;

/// # Arguments
/// * `a` - (int32&)
/// * `b` - (float&)
/// * `c` - (double&)
/// * `d` - (vec4&)
/// * `e` - (int64[]&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef5Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>) {
    unsafe { __cross_call_master_ParamRef5Callback.expect("ParamRef5Callback function was not found")(a, b, c, d, e) }
}
pub type _ParamRef5Callback = unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vec4, &mut Arr<i64>);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef5Callback: Option<_ParamRef5Callback> = None;

/// # Arguments
/// * `a` - (int32&)
/// * `b` - (float&)
/// * `c` - (double&)
/// * `d` - (vec4&)
/// * `e` - (int64[]&)
/// * `f` - (char8&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef6Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8) {
    unsafe { __cross_call_master_ParamRef6Callback.expect("ParamRef6Callback function was not found")(a, b, c, d, e, f) }
}
pub type _ParamRef6Callback = unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vec4, &mut Arr<i64>, &mut i8);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef6Callback: Option<_ParamRef6Callback> = None;

/// # Arguments
/// * `a` - (int32&)
/// * `b` - (float&)
/// * `c` - (double&)
/// * `d` - (vec4&)
/// * `e` - (int64[]&)
/// * `f` - (char8&)
/// * `g` - (string&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef7Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8, g: &mut Str) {
    unsafe { __cross_call_master_ParamRef7Callback.expect("ParamRef7Callback function was not found")(a, b, c, d, e, f, g) }
}
pub type _ParamRef7Callback = unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vec4, &mut Arr<i64>, &mut i8, &mut Str);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef7Callback: Option<_ParamRef7Callback> = None;

/// # Arguments
/// * `a` - (int32&)
/// * `b` - (float&)
/// * `c` - (double&)
/// * `d` - (vec4&)
/// * `e` - (int64[]&)
/// * `f` - (char8&)
/// * `g` - (string&)
/// * `h` - (char16&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef8Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8, g: &mut Str, h: &mut u16) {
    unsafe { __cross_call_master_ParamRef8Callback.expect("ParamRef8Callback function was not found")(a, b, c, d, e, f, g, h) }
}
pub type _ParamRef8Callback = unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vec4, &mut Arr<i64>, &mut i8, &mut Str, &mut u16);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef8Callback: Option<_ParamRef8Callback> = None;

/// # Arguments
/// * `a` - (int32&)
/// * `b` - (float&)
/// * `c` - (double&)
/// * `d` - (vec4&)
/// * `e` - (int64[]&)
/// * `f` - (char8&)
/// * `g` - (string&)
/// * `h` - (char16&)
/// * `k` - (int16&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef9Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8, g: &mut Str, h: &mut u16, k: &mut i16) {
    unsafe { __cross_call_master_ParamRef9Callback.expect("ParamRef9Callback function was not found")(a, b, c, d, e, f, g, h, k) }
}
pub type _ParamRef9Callback = unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vec4, &mut Arr<i64>, &mut i8, &mut Str, &mut u16, &mut i16);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef9Callback: Option<_ParamRef9Callback> = None;

/// # Arguments
/// * `a` - (int32&)
/// * `b` - (float&)
/// * `c` - (double&)
/// * `d` - (vec4&)
/// * `e` - (int64[]&)
/// * `f` - (char8&)
/// * `g` - (string&)
/// * `h` - (char16&)
/// * `k` - (int16&)
/// * `l` - (ptr64&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef10Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8, g: &mut Str, h: &mut u16, k: &mut i16, l: &mut usize) {
    unsafe { __cross_call_master_ParamRef10Callback.expect("ParamRef10Callback function was not found")(a, b, c, d, e, f, g, h, k, l) }
}
pub type _ParamRef10Callback = unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vec4, &mut Arr<i64>, &mut i8, &mut Str, &mut u16, &mut i16, &mut usize);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRef10Callback: Option<_ParamRef10Callback> = None;

/// # Arguments
/// * `p1` - (bool[]&)
/// * `p2` - (char8[]&)
/// * `p3` - (char16[]&)
/// * `p4` - (int8[]&)
/// * `p5` - (int16[]&)
/// * `p6` - (int32[]&)
/// * `p7` - (int64[]&)
/// * `p8` - (uint8[]&)
/// * `p9` - (uint16[]&)
/// * `p10` - (uint32[]&)
/// * `p11` - (uint64[]&)
/// * `p12` - (ptr64[]&)
/// * `p13` - (float[]&)
/// * `p14` - (double[]&)
/// * `p15` - (string[]&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRefVectorsCallback(p1: &mut Arr<bool>, p2: &mut Arr<i8>, p3: &mut Arr<u16>, p4: &mut Arr<i8>, p5: &mut Arr<i16>, p6: &mut Arr<i32>, p7: &mut Arr<i64>, p8: &mut Arr<u8>, p9: &mut Arr<u16>, p10: &mut Arr<u32>, p11: &mut Arr<u64>, p12: &mut Arr<usize>, p13: &mut Arr<f32>, p14: &mut Arr<f64>, p15: &mut Arr<Str>) {
    unsafe { __cross_call_master_ParamRefVectorsCallback.expect("ParamRefVectorsCallback function was not found")(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15) }
}
pub type _ParamRefVectorsCallback = unsafe extern "C" fn(&mut Arr<bool>, &mut Arr<i8>, &mut Arr<u16>, &mut Arr<i8>, &mut Arr<i16>, &mut Arr<i32>, &mut Arr<i64>, &mut Arr<u8>, &mut Arr<u16>, &mut Arr<u32>, &mut Arr<u64>, &mut Arr<usize>, &mut Arr<f32>, &mut Arr<f64>, &mut Arr<Str>);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamRefVectorsCallback: Option<_ParamRefVectorsCallback> = None;

/// # Arguments
/// * `p1` - (bool)
/// * `p2` - (char8)
/// * `p3` - (char16)
/// * `p4` - (int8)
/// * `p5` - (int16)
/// * `p6` - (int32)
/// * `p7` - (int64)
/// * `p8` - (uint8)
/// * `p9` - (uint16)
/// * `p10` - (uint32)
/// * `p11` - (uint64)
/// * `p12` - (ptr64)
/// * `p13` - (float)
/// * `p14` - (double)
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn ParamAllPrimitivesCallback(p1: bool, p2: i8, p3: u16, p4: i8, p5: i16, p6: i32, p7: i64, p8: u8, p9: u16, p10: u32, p11: u64, p12: usize, p13: f32, p14: f64) -> i64 {
    unsafe { __cross_call_master_ParamAllPrimitivesCallback.expect("ParamAllPrimitivesCallback function was not found")(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14) }
}
pub type _ParamAllPrimitivesCallback = unsafe extern "C" fn(bool, i8, u16, i8, i16, i32, i64, u8, u16, u32, u64, usize, f32, f64) -> i64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamAllPrimitivesCallback: Option<_ParamAllPrimitivesCallback> = None;

/// # Arguments
/// * `p1` - (int32)
/// * `p2` - (int32[])
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn ParamEnumCallback(p1: Example, p2: &Arr<Example>) -> i32 {
    unsafe { __cross_call_master_ParamEnumCallback.expect("ParamEnumCallback function was not found")(p1, p2) }
}
pub type _ParamEnumCallback = unsafe extern "C" fn(Example, &Arr<Example>) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamEnumCallback: Option<_ParamEnumCallback> = None;

/// # Arguments
/// * `p1` - (int32&)
/// * `p2` - (int32[]&)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn ParamEnumRefCallback(p1: &mut Example, p2: &mut Arr<Example>) -> i32 {
    unsafe { __cross_call_master_ParamEnumRefCallback.expect("ParamEnumRefCallback function was not found")(p1, p2) }
}
pub type _ParamEnumRefCallback = unsafe extern "C" fn(&mut Example, &mut Arr<Example>) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamEnumRefCallback: Option<_ParamEnumRefCallback> = None;

/// # Arguments
/// * `p1` - (any)
/// * `p2` - (any[])
#[allow(dead_code, non_snake_case)]
pub fn ParamVariantCallback(p1: &Var, p2: &Arr<Var>) {
    unsafe { __cross_call_master_ParamVariantCallback.expect("ParamVariantCallback function was not found")(p1, p2) }
}
pub type _ParamVariantCallback = unsafe extern "C" fn(&Var, &Arr<Var>);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamVariantCallback: Option<_ParamVariantCallback> = None;

/// # Arguments
/// * `p1` - (any&)
/// * `p2` - (any[]&)
#[allow(dead_code, non_snake_case)]
pub fn ParamVariantRefCallback(p1: &mut Var, p2: &mut Arr<Var>) {
    unsafe { __cross_call_master_ParamVariantRefCallback.expect("ParamVariantRefCallback function was not found")(p1, p2) }
}
pub type _ParamVariantRefCallback = unsafe extern "C" fn(&mut Var, &mut Arr<Var>);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ParamVariantRefCallback: Option<_ParamVariantRefCallback> = None;

/// # Arguments
/// * `func` - (function)
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVoidCallback(func: FuncVoid) {
    unsafe { __cross_call_master_CallFuncVoidCallback.expect("CallFuncVoidCallback function was not found")(func) }
}
pub type _CallFuncVoidCallback = unsafe extern "C" fn(FuncVoid);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncVoidCallback: Option<_CallFuncVoidCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn CallFuncBoolCallback(func: FuncBool) -> bool {
    unsafe { __cross_call_master_CallFuncBoolCallback.expect("CallFuncBoolCallback function was not found")(func) }
}
pub type _CallFuncBoolCallback = unsafe extern "C" fn(FuncBool) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncBoolCallback: Option<_CallFuncBoolCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char8
#[allow(dead_code, non_snake_case)]
pub fn CallFuncChar8Callback(func: FuncChar8) -> i8 {
    unsafe { __cross_call_master_CallFuncChar8Callback.expect("CallFuncChar8Callback function was not found")(func) }
}
pub type _CallFuncChar8Callback = unsafe extern "C" fn(FuncChar8) -> i8;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncChar8Callback: Option<_CallFuncChar8Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char16
#[allow(dead_code, non_snake_case)]
pub fn CallFuncChar16Callback(func: FuncChar16) -> u16 {
    unsafe { __cross_call_master_CallFuncChar16Callback.expect("CallFuncChar16Callback function was not found")(func) }
}
pub type _CallFuncChar16Callback = unsafe extern "C" fn(FuncChar16) -> u16;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncChar16Callback: Option<_CallFuncChar16Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int8
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt8Callback(func: FuncInt8) -> i8 {
    unsafe { __cross_call_master_CallFuncInt8Callback.expect("CallFuncInt8Callback function was not found")(func) }
}
pub type _CallFuncInt8Callback = unsafe extern "C" fn(FuncInt8) -> i8;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncInt8Callback: Option<_CallFuncInt8Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int16
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt16Callback(func: FuncInt16) -> i16 {
    unsafe { __cross_call_master_CallFuncInt16Callback.expect("CallFuncInt16Callback function was not found")(func) }
}
pub type _CallFuncInt16Callback = unsafe extern "C" fn(FuncInt16) -> i16;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncInt16Callback: Option<_CallFuncInt16Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt32Callback(func: FuncInt32) -> i32 {
    unsafe { __cross_call_master_CallFuncInt32Callback.expect("CallFuncInt32Callback function was not found")(func) }
}
pub type _CallFuncInt32Callback = unsafe extern "C" fn(FuncInt32) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncInt32Callback: Option<_CallFuncInt32Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt64Callback(func: FuncInt64) -> i64 {
    unsafe { __cross_call_master_CallFuncInt64Callback.expect("CallFuncInt64Callback function was not found")(func) }
}
pub type _CallFuncInt64Callback = unsafe extern "C" fn(FuncInt64) -> i64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncInt64Callback: Option<_CallFuncInt64Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint8
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt8Callback(func: FuncUInt8) -> u8 {
    unsafe { __cross_call_master_CallFuncUInt8Callback.expect("CallFuncUInt8Callback function was not found")(func) }
}
pub type _CallFuncUInt8Callback = unsafe extern "C" fn(FuncUInt8) -> u8;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncUInt8Callback: Option<_CallFuncUInt8Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint16
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt16Callback(func: FuncUInt16) -> u16 {
    unsafe { __cross_call_master_CallFuncUInt16Callback.expect("CallFuncUInt16Callback function was not found")(func) }
}
pub type _CallFuncUInt16Callback = unsafe extern "C" fn(FuncUInt16) -> u16;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncUInt16Callback: Option<_CallFuncUInt16Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint32
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt32Callback(func: FuncUInt32) -> u32 {
    unsafe { __cross_call_master_CallFuncUInt32Callback.expect("CallFuncUInt32Callback function was not found")(func) }
}
pub type _CallFuncUInt32Callback = unsafe extern "C" fn(FuncUInt32) -> u32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncUInt32Callback: Option<_CallFuncUInt32Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint64
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt64Callback(func: FuncUInt64) -> u64 {
    unsafe { __cross_call_master_CallFuncUInt64Callback.expect("CallFuncUInt64Callback function was not found")(func) }
}
pub type _CallFuncUInt64Callback = unsafe extern "C" fn(FuncUInt64) -> u64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncUInt64Callback: Option<_CallFuncUInt64Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CallFuncPtrCallback(func: FuncPtr) -> usize {
    unsafe { __cross_call_master_CallFuncPtrCallback.expect("CallFuncPtrCallback function was not found")(func) }
}
pub type _CallFuncPtrCallback = unsafe extern "C" fn(FuncPtr) -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncPtrCallback: Option<_CallFuncPtrCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// float
#[allow(dead_code, non_snake_case)]
pub fn CallFuncFloatCallback(func: FuncFloat) -> f32 {
    unsafe { __cross_call_master_CallFuncFloatCallback.expect("CallFuncFloatCallback function was not found")(func) }
}
pub type _CallFuncFloatCallback = unsafe extern "C" fn(FuncFloat) -> f32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncFloatCallback: Option<_CallFuncFloatCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// double
#[allow(dead_code, non_snake_case)]
pub fn CallFuncDoubleCallback(func: FuncDouble) -> f64 {
    unsafe { __cross_call_master_CallFuncDoubleCallback.expect("CallFuncDoubleCallback function was not found")(func) }
}
pub type _CallFuncDoubleCallback = unsafe extern "C" fn(FuncDouble) -> f64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncDoubleCallback: Option<_CallFuncDoubleCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFuncStringCallback(func: FuncString) -> Str {
    unsafe { __cross_call_master_CallFuncStringCallback.expect("CallFuncStringCallback function was not found")(func) }
}
pub type _CallFuncStringCallback = unsafe extern "C" fn(FuncString) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncStringCallback: Option<_CallFuncStringCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// any
#[allow(dead_code, non_snake_case)]
pub fn CallFuncAnyCallback(func: FuncAny) -> Var {
    unsafe { __cross_call_master_CallFuncAnyCallback.expect("CallFuncAnyCallback function was not found")(func) }
}
pub type _CallFuncAnyCallback = unsafe extern "C" fn(FuncAny) -> Var;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncAnyCallback: Option<_CallFuncAnyCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CallFuncFunctionCallback(func: FuncFunction) -> usize {
    unsafe { __cross_call_master_CallFuncFunctionCallback.expect("CallFuncFunctionCallback function was not found")(func) }
}
pub type _CallFuncFunctionCallback = unsafe extern "C" fn(FuncFunction) -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncFunctionCallback: Option<_CallFuncFunctionCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// bool[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncBoolVectorCallback(func: FuncBoolVector) -> Arr<bool> {
    unsafe { __cross_call_master_CallFuncBoolVectorCallback.expect("CallFuncBoolVectorCallback function was not found")(func) }
}
pub type _CallFuncBoolVectorCallback = unsafe extern "C" fn(FuncBoolVector) -> Arr<bool>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncBoolVectorCallback: Option<_CallFuncBoolVectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char8[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncChar8VectorCallback(func: FuncChar8Vector) -> Arr<i8> {
    unsafe { __cross_call_master_CallFuncChar8VectorCallback.expect("CallFuncChar8VectorCallback function was not found")(func) }
}
pub type _CallFuncChar8VectorCallback = unsafe extern "C" fn(FuncChar8Vector) -> Arr<i8>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncChar8VectorCallback: Option<_CallFuncChar8VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char16[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncChar16VectorCallback(func: FuncChar16Vector) -> Arr<u16> {
    unsafe { __cross_call_master_CallFuncChar16VectorCallback.expect("CallFuncChar16VectorCallback function was not found")(func) }
}
pub type _CallFuncChar16VectorCallback = unsafe extern "C" fn(FuncChar16Vector) -> Arr<u16>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncChar16VectorCallback: Option<_CallFuncChar16VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int8[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt8VectorCallback(func: FuncInt8Vector) -> Arr<i8> {
    unsafe { __cross_call_master_CallFuncInt8VectorCallback.expect("CallFuncInt8VectorCallback function was not found")(func) }
}
pub type _CallFuncInt8VectorCallback = unsafe extern "C" fn(FuncInt8Vector) -> Arr<i8>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncInt8VectorCallback: Option<_CallFuncInt8VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int16[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt16VectorCallback(func: FuncInt16Vector) -> Arr<i16> {
    unsafe { __cross_call_master_CallFuncInt16VectorCallback.expect("CallFuncInt16VectorCallback function was not found")(func) }
}
pub type _CallFuncInt16VectorCallback = unsafe extern "C" fn(FuncInt16Vector) -> Arr<i16>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncInt16VectorCallback: Option<_CallFuncInt16VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int32[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt32VectorCallback(func: FuncInt32Vector) -> Arr<i32> {
    unsafe { __cross_call_master_CallFuncInt32VectorCallback.expect("CallFuncInt32VectorCallback function was not found")(func) }
}
pub type _CallFuncInt32VectorCallback = unsafe extern "C" fn(FuncInt32Vector) -> Arr<i32>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncInt32VectorCallback: Option<_CallFuncInt32VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int64[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt64VectorCallback(func: FuncInt64Vector) -> Arr<i64> {
    unsafe { __cross_call_master_CallFuncInt64VectorCallback.expect("CallFuncInt64VectorCallback function was not found")(func) }
}
pub type _CallFuncInt64VectorCallback = unsafe extern "C" fn(FuncInt64Vector) -> Arr<i64>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncInt64VectorCallback: Option<_CallFuncInt64VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint8[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt8VectorCallback(func: FuncUInt8Vector) -> Arr<u8> {
    unsafe { __cross_call_master_CallFuncUInt8VectorCallback.expect("CallFuncUInt8VectorCallback function was not found")(func) }
}
pub type _CallFuncUInt8VectorCallback = unsafe extern "C" fn(FuncUInt8Vector) -> Arr<u8>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncUInt8VectorCallback: Option<_CallFuncUInt8VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint16[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt16VectorCallback(func: FuncUInt16Vector) -> Arr<u16> {
    unsafe { __cross_call_master_CallFuncUInt16VectorCallback.expect("CallFuncUInt16VectorCallback function was not found")(func) }
}
pub type _CallFuncUInt16VectorCallback = unsafe extern "C" fn(FuncUInt16Vector) -> Arr<u16>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncUInt16VectorCallback: Option<_CallFuncUInt16VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint32[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt32VectorCallback(func: FuncUInt32Vector) -> Arr<u32> {
    unsafe { __cross_call_master_CallFuncUInt32VectorCallback.expect("CallFuncUInt32VectorCallback function was not found")(func) }
}
pub type _CallFuncUInt32VectorCallback = unsafe extern "C" fn(FuncUInt32Vector) -> Arr<u32>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncUInt32VectorCallback: Option<_CallFuncUInt32VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint64[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt64VectorCallback(func: FuncUInt64Vector) -> Arr<u64> {
    unsafe { __cross_call_master_CallFuncUInt64VectorCallback.expect("CallFuncUInt64VectorCallback function was not found")(func) }
}
pub type _CallFuncUInt64VectorCallback = unsafe extern "C" fn(FuncUInt64Vector) -> Arr<u64>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncUInt64VectorCallback: Option<_CallFuncUInt64VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncPtrVectorCallback(func: FuncPtrVector) -> Arr<usize> {
    unsafe { __cross_call_master_CallFuncPtrVectorCallback.expect("CallFuncPtrVectorCallback function was not found")(func) }
}
pub type _CallFuncPtrVectorCallback = unsafe extern "C" fn(FuncPtrVector) -> Arr<usize>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncPtrVectorCallback: Option<_CallFuncPtrVectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// float[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncFloatVectorCallback(func: FuncFloatVector) -> Arr<f32> {
    unsafe { __cross_call_master_CallFuncFloatVectorCallback.expect("CallFuncFloatVectorCallback function was not found")(func) }
}
pub type _CallFuncFloatVectorCallback = unsafe extern "C" fn(FuncFloatVector) -> Arr<f32>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncFloatVectorCallback: Option<_CallFuncFloatVectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// double[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncDoubleVectorCallback(func: FuncDoubleVector) -> Arr<f64> {
    unsafe { __cross_call_master_CallFuncDoubleVectorCallback.expect("CallFuncDoubleVectorCallback function was not found")(func) }
}
pub type _CallFuncDoubleVectorCallback = unsafe extern "C" fn(FuncDoubleVector) -> Arr<f64>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncDoubleVectorCallback: Option<_CallFuncDoubleVectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncStringVectorCallback(func: FuncStringVector) -> Arr<Str> {
    unsafe { __cross_call_master_CallFuncStringVectorCallback.expect("CallFuncStringVectorCallback function was not found")(func) }
}
pub type _CallFuncStringVectorCallback = unsafe extern "C" fn(FuncStringVector) -> Arr<Str>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncStringVectorCallback: Option<_CallFuncStringVectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// any[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncAnyVectorCallback(func: FuncAnyVector) -> Arr<Var> {
    unsafe { __cross_call_master_CallFuncAnyVectorCallback.expect("CallFuncAnyVectorCallback function was not found")(func) }
}
pub type _CallFuncAnyVectorCallback = unsafe extern "C" fn(FuncAnyVector) -> Arr<Var>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncAnyVectorCallback: Option<_CallFuncAnyVectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec2[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec2VectorCallback(func: FuncVec2Vector) -> Arr<Vec2> {
    unsafe { __cross_call_master_CallFuncVec2VectorCallback.expect("CallFuncVec2VectorCallback function was not found")(func) }
}
pub type _CallFuncVec2VectorCallback = unsafe extern "C" fn(FuncVec2Vector) -> Arr<Vec2>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncVec2VectorCallback: Option<_CallFuncVec2VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec3[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec3VectorCallback(func: FuncVec3Vector) -> Arr<Vec3> {
    unsafe { __cross_call_master_CallFuncVec3VectorCallback.expect("CallFuncVec3VectorCallback function was not found")(func) }
}
pub type _CallFuncVec3VectorCallback = unsafe extern "C" fn(FuncVec3Vector) -> Arr<Vec3>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncVec3VectorCallback: Option<_CallFuncVec3VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec4[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec4VectorCallback(func: FuncVec4Vector) -> Arr<Vec4> {
    unsafe { __cross_call_master_CallFuncVec4VectorCallback.expect("CallFuncVec4VectorCallback function was not found")(func) }
}
pub type _CallFuncVec4VectorCallback = unsafe extern "C" fn(FuncVec4Vector) -> Arr<Vec4>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncVec4VectorCallback: Option<_CallFuncVec4VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// mat4x4[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncMat4x4VectorCallback(func: FuncMat4x4Vector) -> Arr<Mat4x4> {
    unsafe { __cross_call_master_CallFuncMat4x4VectorCallback.expect("CallFuncMat4x4VectorCallback function was not found")(func) }
}
pub type _CallFuncMat4x4VectorCallback = unsafe extern "C" fn(FuncMat4x4Vector) -> Arr<Mat4x4>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncMat4x4VectorCallback: Option<_CallFuncMat4x4VectorCallback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec2
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec2Callback(func: FuncVec2) -> Vec2 {
    unsafe { __cross_call_master_CallFuncVec2Callback.expect("CallFuncVec2Callback function was not found")(func) }
}
pub type _CallFuncVec2Callback = unsafe extern "C" fn(FuncVec2) -> Vec2;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncVec2Callback: Option<_CallFuncVec2Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec3
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec3Callback(func: FuncVec3) -> Vec3 {
    unsafe { __cross_call_master_CallFuncVec3Callback.expect("CallFuncVec3Callback function was not found")(func) }
}
pub type _CallFuncVec3Callback = unsafe extern "C" fn(FuncVec3) -> Vec3;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncVec3Callback: Option<_CallFuncVec3Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec4
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec4Callback(func: FuncVec4) -> Vec4 {
    unsafe { __cross_call_master_CallFuncVec4Callback.expect("CallFuncVec4Callback function was not found")(func) }
}
pub type _CallFuncVec4Callback = unsafe extern "C" fn(FuncVec4) -> Vec4;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncVec4Callback: Option<_CallFuncVec4Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// mat4x4
#[allow(dead_code, non_snake_case)]
pub fn CallFuncMat4x4Callback(func: FuncMat4x4) -> Mat4x4 {
    unsafe { __cross_call_master_CallFuncMat4x4Callback.expect("CallFuncMat4x4Callback function was not found")(func) }
}
pub type _CallFuncMat4x4Callback = unsafe extern "C" fn(FuncMat4x4) -> Mat4x4;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncMat4x4Callback: Option<_CallFuncMat4x4Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn CallFunc1Callback(func: Func1) -> i32 {
    unsafe { __cross_call_master_CallFunc1Callback.expect("CallFunc1Callback function was not found")(func) }
}
pub type _CallFunc1Callback = unsafe extern "C" fn(Func1) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc1Callback: Option<_CallFunc1Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char8
#[allow(dead_code, non_snake_case)]
pub fn CallFunc2Callback(func: Func2) -> i8 {
    unsafe { __cross_call_master_CallFunc2Callback.expect("CallFunc2Callback function was not found")(func) }
}
pub type _CallFunc2Callback = unsafe extern "C" fn(Func2) -> i8;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc2Callback: Option<_CallFunc2Callback> = None;

/// # Arguments
/// * `func` - (function)
#[allow(dead_code, non_snake_case)]
pub fn CallFunc3Callback(func: Func3) {
    unsafe { __cross_call_master_CallFunc3Callback.expect("CallFunc3Callback function was not found")(func) }
}
pub type _CallFunc3Callback = unsafe extern "C" fn(Func3);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc3Callback: Option<_CallFunc3Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec4
#[allow(dead_code, non_snake_case)]
pub fn CallFunc4Callback(func: Func4) -> Vec4 {
    unsafe { __cross_call_master_CallFunc4Callback.expect("CallFunc4Callback function was not found")(func) }
}
pub type _CallFunc4Callback = unsafe extern "C" fn(Func4) -> Vec4;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc4Callback: Option<_CallFunc4Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn CallFunc5Callback(func: Func5) -> bool {
    unsafe { __cross_call_master_CallFunc5Callback.expect("CallFunc5Callback function was not found")(func) }
}
pub type _CallFunc5Callback = unsafe extern "C" fn(Func5) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc5Callback: Option<_CallFunc5Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn CallFunc6Callback(func: Func6) -> i64 {
    unsafe { __cross_call_master_CallFunc6Callback.expect("CallFunc6Callback function was not found")(func) }
}
pub type _CallFunc6Callback = unsafe extern "C" fn(Func6) -> i64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc6Callback: Option<_CallFunc6Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// double
#[allow(dead_code, non_snake_case)]
pub fn CallFunc7Callback(func: Func7) -> f64 {
    unsafe { __cross_call_master_CallFunc7Callback.expect("CallFunc7Callback function was not found")(func) }
}
pub type _CallFunc7Callback = unsafe extern "C" fn(Func7) -> f64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc7Callback: Option<_CallFunc7Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// mat4x4
#[allow(dead_code, non_snake_case)]
pub fn CallFunc8Callback(func: Func8) -> Mat4x4 {
    unsafe { __cross_call_master_CallFunc8Callback.expect("CallFunc8Callback function was not found")(func) }
}
pub type _CallFunc8Callback = unsafe extern "C" fn(Func8) -> Mat4x4;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc8Callback: Option<_CallFunc8Callback> = None;

/// # Arguments
/// * `func` - (function)
#[allow(dead_code, non_snake_case)]
pub fn CallFunc9Callback(func: Func9) {
    unsafe { __cross_call_master_CallFunc9Callback.expect("CallFunc9Callback function was not found")(func) }
}
pub type _CallFunc9Callback = unsafe extern "C" fn(Func9);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc9Callback: Option<_CallFunc9Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint32
#[allow(dead_code, non_snake_case)]
pub fn CallFunc10Callback(func: Func10) -> u32 {
    unsafe { __cross_call_master_CallFunc10Callback.expect("CallFunc10Callback function was not found")(func) }
}
pub type _CallFunc10Callback = unsafe extern "C" fn(Func10) -> u32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc10Callback: Option<_CallFunc10Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CallFunc11Callback(func: Func11) -> usize {
    unsafe { __cross_call_master_CallFunc11Callback.expect("CallFunc11Callback function was not found")(func) }
}
pub type _CallFunc11Callback = unsafe extern "C" fn(Func11) -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc11Callback: Option<_CallFunc11Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn CallFunc12Callback(func: Func12) -> bool {
    unsafe { __cross_call_master_CallFunc12Callback.expect("CallFunc12Callback function was not found")(func) }
}
pub type _CallFunc12Callback = unsafe extern "C" fn(Func12) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc12Callback: Option<_CallFunc12Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc13Callback(func: Func13) -> Str {
    unsafe { __cross_call_master_CallFunc13Callback.expect("CallFunc13Callback function was not found")(func) }
}
pub type _CallFunc13Callback = unsafe extern "C" fn(Func13) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc13Callback: Option<_CallFunc13Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string[]
#[allow(dead_code, non_snake_case)]
pub fn CallFunc14Callback(func: Func14) -> Arr<Str> {
    unsafe { __cross_call_master_CallFunc14Callback.expect("CallFunc14Callback function was not found")(func) }
}
pub type _CallFunc14Callback = unsafe extern "C" fn(Func14) -> Arr<Str>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc14Callback: Option<_CallFunc14Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int16
#[allow(dead_code, non_snake_case)]
pub fn CallFunc15Callback(func: Func15) -> i16 {
    unsafe { __cross_call_master_CallFunc15Callback.expect("CallFunc15Callback function was not found")(func) }
}
pub type _CallFunc15Callback = unsafe extern "C" fn(Func15) -> i16;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc15Callback: Option<_CallFunc15Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CallFunc16Callback(func: Func16) -> usize {
    unsafe { __cross_call_master_CallFunc16Callback.expect("CallFunc16Callback function was not found")(func) }
}
pub type _CallFunc16Callback = unsafe extern "C" fn(Func16) -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc16Callback: Option<_CallFunc16Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc17Callback(func: Func17) -> Str {
    unsafe { __cross_call_master_CallFunc17Callback.expect("CallFunc17Callback function was not found")(func) }
}
pub type _CallFunc17Callback = unsafe extern "C" fn(Func17) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc17Callback: Option<_CallFunc17Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc18Callback(func: Func18) -> Str {
    unsafe { __cross_call_master_CallFunc18Callback.expect("CallFunc18Callback function was not found")(func) }
}
pub type _CallFunc18Callback = unsafe extern "C" fn(Func18) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc18Callback: Option<_CallFunc18Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc19Callback(func: Func19) -> Str {
    unsafe { __cross_call_master_CallFunc19Callback.expect("CallFunc19Callback function was not found")(func) }
}
pub type _CallFunc19Callback = unsafe extern "C" fn(Func19) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc19Callback: Option<_CallFunc19Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc20Callback(func: Func20) -> Str {
    unsafe { __cross_call_master_CallFunc20Callback.expect("CallFunc20Callback function was not found")(func) }
}
pub type _CallFunc20Callback = unsafe extern "C" fn(Func20) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc20Callback: Option<_CallFunc20Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc21Callback(func: Func21) -> Str {
    unsafe { __cross_call_master_CallFunc21Callback.expect("CallFunc21Callback function was not found")(func) }
}
pub type _CallFunc21Callback = unsafe extern "C" fn(Func21) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc21Callback: Option<_CallFunc21Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc22Callback(func: Func22) -> Str {
    unsafe { __cross_call_master_CallFunc22Callback.expect("CallFunc22Callback function was not found")(func) }
}
pub type _CallFunc22Callback = unsafe extern "C" fn(Func22) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc22Callback: Option<_CallFunc22Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc23Callback(func: Func23) -> Str {
    unsafe { __cross_call_master_CallFunc23Callback.expect("CallFunc23Callback function was not found")(func) }
}
pub type _CallFunc23Callback = unsafe extern "C" fn(Func23) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc23Callback: Option<_CallFunc23Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc24Callback(func: Func24) -> Str {
    unsafe { __cross_call_master_CallFunc24Callback.expect("CallFunc24Callback function was not found")(func) }
}
pub type _CallFunc24Callback = unsafe extern "C" fn(Func24) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc24Callback: Option<_CallFunc24Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc25Callback(func: Func25) -> Str {
    unsafe { __cross_call_master_CallFunc25Callback.expect("CallFunc25Callback function was not found")(func) }
}
pub type _CallFunc25Callback = unsafe extern "C" fn(Func25) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc25Callback: Option<_CallFunc25Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc26Callback(func: Func26) -> Str {
    unsafe { __cross_call_master_CallFunc26Callback.expect("CallFunc26Callback function was not found")(func) }
}
pub type _CallFunc26Callback = unsafe extern "C" fn(Func26) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc26Callback: Option<_CallFunc26Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc27Callback(func: Func27) -> Str {
    unsafe { __cross_call_master_CallFunc27Callback.expect("CallFunc27Callback function was not found")(func) }
}
pub type _CallFunc27Callback = unsafe extern "C" fn(Func27) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc27Callback: Option<_CallFunc27Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc28Callback(func: Func28) -> Str {
    unsafe { __cross_call_master_CallFunc28Callback.expect("CallFunc28Callback function was not found")(func) }
}
pub type _CallFunc28Callback = unsafe extern "C" fn(Func28) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc28Callback: Option<_CallFunc28Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc29Callback(func: Func29) -> Str {
    unsafe { __cross_call_master_CallFunc29Callback.expect("CallFunc29Callback function was not found")(func) }
}
pub type _CallFunc29Callback = unsafe extern "C" fn(Func29) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc29Callback: Option<_CallFunc29Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc30Callback(func: Func30) -> Str {
    unsafe { __cross_call_master_CallFunc30Callback.expect("CallFunc30Callback function was not found")(func) }
}
pub type _CallFunc30Callback = unsafe extern "C" fn(Func30) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc30Callback: Option<_CallFunc30Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc31Callback(func: Func31) -> Str {
    unsafe { __cross_call_master_CallFunc31Callback.expect("CallFunc31Callback function was not found")(func) }
}
pub type _CallFunc31Callback = unsafe extern "C" fn(Func31) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc31Callback: Option<_CallFunc31Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc32Callback(func: Func32) -> Str {
    unsafe { __cross_call_master_CallFunc32Callback.expect("CallFunc32Callback function was not found")(func) }
}
pub type _CallFunc32Callback = unsafe extern "C" fn(Func32) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc32Callback: Option<_CallFunc32Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc33Callback(func: Func33) -> Str {
    unsafe { __cross_call_master_CallFunc33Callback.expect("CallFunc33Callback function was not found")(func) }
}
pub type _CallFunc33Callback = unsafe extern "C" fn(Func33) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFunc33Callback: Option<_CallFunc33Callback> = None;

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFuncEnumCallback(func: FuncEnum) -> Str {
    unsafe { __cross_call_master_CallFuncEnumCallback.expect("CallFuncEnumCallback function was not found")(func) }
}
pub type _CallFuncEnumCallback = unsafe extern "C" fn(FuncEnum) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CallFuncEnumCallback: Option<_CallFuncEnumCallback> = None;

#[derive(Debug)]
pub enum ResourceHandleError {
    EmptyHandle,
}

impl std::fmt::Display for ResourceHandleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceHandleError::EmptyHandle => write!(f, "empty handle"),
        }
    }
}

impl std::error::Error for ResourceHandleError {}

/// RAII wrapper for ResourceHandle pointer
#[derive(Debug)]
pub struct ResourceHandle {
    handle: usize,
    ownership: Ownership,
}

impl ResourceHandle {
    /// # Arguments
    /// * `id` - (int32)
    /// * `name` - (string)
    #[allow(dead_code, non_snake_case)]
    pub fn new_ResourceHandleCreate(id: i32, name: &Str) -> Result<Self, ResourceHandleError> {
        let h = crate::cross_call_master::ResourceHandleCreate(id, name);
        if h == 0 {
            return Err(ResourceHandleError::EmptyHandle);
        }
        Ok(Self {
            handle: h,
            ownership: Ownership::Owned,
        })
    }

    #[allow(dead_code, non_snake_case)]
    pub fn new_ResourceHandleCreateDefault() -> Result<Self, ResourceHandleError> {
        let h = crate::cross_call_master::ResourceHandleCreateDefault();
        if h == 0 {
            return Err(ResourceHandleError::EmptyHandle);
        }
        Ok(Self {
            handle: h,
            ownership: Ownership::Owned,
        })
    }

    /// Construct from raw handle with specified ownership
    #[allow(dead_code)]
    pub unsafe fn from_raw(handle: usize, ownership: Ownership) -> Self {
        Self { handle, ownership }
    }

    /// Returns the underlying handle
    #[allow(dead_code)]
    pub fn get(&self) -> usize {
        self.handle
    }

    /// Release ownership and return the handle. Wrapper becomes empty & borrowed.
    #[allow(dead_code)]
    pub fn release(&mut self) -> usize {
        let h = self.handle;
        self.handle = 0;
        self.ownership = Ownership::Borrowed;
        h
    }

    /// Destroys and resets the handle
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        if self.handle != 0 && self.ownership == Ownership::Owned {
            crate::cross_call_master::ResourceHandleDestroy(self.handle);
        }
        self.handle = 0;
        self.ownership = Ownership::Borrowed;
    }

    /// Swaps two ResourceHandle instances
    #[allow(dead_code)]
    pub fn swap(&mut self, other: &mut ResourceHandle) {
        std::mem::swap(&mut self.handle, &mut other.handle);
        std::mem::swap(&mut self.ownership, &mut other.ownership);
    }

    /// Returns true if handle is valid (not empty)
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.handle != 0
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetId(&self) -> Result<i32, ResourceHandleError> {
        if self.handle == 0 {
            return Err(ResourceHandleError::EmptyHandle);
        }
        Ok(crate::cross_call_master::ResourceHandleGetId(self.handle))
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetName(&self) -> Result<Str, ResourceHandleError> {
        if self.handle == 0 {
            return Err(ResourceHandleError::EmptyHandle);
        }
        Ok(crate::cross_call_master::ResourceHandleGetName(self.handle))
    }

    /// # Arguments
    /// * `name` - (string)
    #[allow(dead_code, non_snake_case)]
    pub fn SetName(&self, name: &Str) -> Result<(), ResourceHandleError> {
        if self.handle == 0 {
            return Err(ResourceHandleError::EmptyHandle);
        }
        crate::cross_call_master::ResourceHandleSetName(self.handle, name);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn IncrementCounter(&self) -> Result<(), ResourceHandleError> {
        if self.handle == 0 {
            return Err(ResourceHandleError::EmptyHandle);
        }
        crate::cross_call_master::ResourceHandleIncrementCounter(self.handle);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetCounter(&self) -> Result<i32, ResourceHandleError> {
        if self.handle == 0 {
            return Err(ResourceHandleError::EmptyHandle);
        }
        Ok(crate::cross_call_master::ResourceHandleGetCounter(self.handle))
    }

    /// # Arguments
    /// * `value` - (float)
    #[allow(dead_code, non_snake_case)]
    pub fn AddData(&self, value: f32) -> Result<(), ResourceHandleError> {
        if self.handle == 0 {
            return Err(ResourceHandleError::EmptyHandle);
        }
        crate::cross_call_master::ResourceHandleAddData(self.handle, value);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetData(&self) -> Result<Arr<f32>, ResourceHandleError> {
        if self.handle == 0 {
            return Err(ResourceHandleError::EmptyHandle);
        }
        Ok(crate::cross_call_master::ResourceHandleGetData(self.handle))
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetAliveCount() -> i32 {
        crate::cross_call_master::ResourceHandleGetAliveCount()
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetTotalCreated() -> i32 {
        crate::cross_call_master::ResourceHandleGetTotalCreated()
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetTotalDestroyed() -> i32 {
        crate::cross_call_master::ResourceHandleGetTotalDestroyed()
    }

}

impl Drop for ResourceHandle {
    fn drop(&mut self) {
        if self.handle != 0 && self.ownership == Ownership::Owned {
            crate::cross_call_master::ResourceHandleDestroy(self.handle);
        }
    }
}

impl std::cmp::PartialEq for ResourceHandle {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl std::cmp::Eq for ResourceHandle {}
impl std::cmp::PartialOrd for ResourceHandle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.handle).partial_cmp(&(other.handle))
    }
}
impl std::cmp::Ord for ResourceHandle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.handle).cmp(&(other.handle))
    }
}


#[derive(Debug)]
pub enum CounterError {
    EmptyHandle,
}

impl std::fmt::Display for CounterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CounterError::EmptyHandle => write!(f, "empty handle"),
        }
    }
}

impl std::error::Error for CounterError {}

#[derive(Debug, Clone, Copy)]
pub struct Counter {
    handle: usize,
}

impl Counter {
    /// # Arguments
    /// * `initialValue` - (int64)
    #[allow(dead_code, non_snake_case)]
    pub fn new_CounterCreate(initialValue: i64) -> Result<Self, CounterError> {
        let h = crate::cross_call_master::CounterCreate(initialValue);
        if h == 0 {
            return Err(CounterError::EmptyHandle);
        }
        Ok(Self {
            handle: h,
        })
    }

    #[allow(dead_code, non_snake_case)]
    pub fn new_CounterCreateZero() -> Result<Self, CounterError> {
        let h = crate::cross_call_master::CounterCreateZero();
        if h == 0 {
            return Err(CounterError::EmptyHandle);
        }
        Ok(Self {
            handle: h,
        })
    }

    /// Construct from raw handle (does not assume ownership)
    #[allow(dead_code)]
    pub unsafe fn from_raw(handle: usize) -> Self {
        Self { handle }
    }

    /// Returns the underlying handle
    #[allow(dead_code)]
    pub fn get(&self) -> usize {
        self.handle
    }

    /// Release ownership and return the handle. Wrapper becomes empty & borrowed.
    #[allow(dead_code)]
    pub fn release(&mut self) -> usize {
        let h = self.handle;
        self.handle = 0;
        h
    }

    /// Destroys and resets the handle
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.handle = 0;
    }

    /// Swaps two Counter instances
    #[allow(dead_code)]
    pub fn swap(&mut self, other: &mut Counter) {
        std::mem::swap(&mut self.handle, &mut other.handle);
    }

    /// Returns true if handle is valid (not empty)
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.handle != 0
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetValue(&self) -> Result<i64, CounterError> {
        if self.handle == 0 {
            return Err(CounterError::EmptyHandle);
        }
        Ok(crate::cross_call_master::CounterGetValue(self.handle))
    }

    /// # Arguments
    /// * `value` - (int64)
    #[allow(dead_code, non_snake_case)]
    pub fn SetValue(&self, value: i64) -> Result<(), CounterError> {
        if self.handle == 0 {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterSetValue(self.handle, value);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn Increment(&self) -> Result<(), CounterError> {
        if self.handle == 0 {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterIncrement(self.handle);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn Decrement(&self) -> Result<(), CounterError> {
        if self.handle == 0 {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterDecrement(self.handle);
        Ok(())
    }

    /// # Arguments
    /// * `amount` - (int64)
    #[allow(dead_code, non_snake_case)]
    pub fn Add(&self, amount: i64) -> Result<(), CounterError> {
        if self.handle == 0 {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterAdd(self.handle, amount);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn Reset(&self) -> Result<(), CounterError> {
        if self.handle == 0 {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterReset(self.handle);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn IsPositive(&self) -> Result<bool, CounterError> {
        if self.handle == 0 {
            return Err(CounterError::EmptyHandle);
        }
        Ok(crate::cross_call_master::CounterIsPositive(self.handle))
    }

    /// # Arguments
    /// * `value1` - (int64)
    /// * `value2` - (int64)
    #[allow(dead_code, non_snake_case)]
    pub fn Compare(value1: i64, value2: i64) -> i32 {
        crate::cross_call_master::CounterCompare(value1, value2)
    }

    /// # Arguments
    /// * `values` - (int64[])
    #[allow(dead_code, non_snake_case)]
    pub fn Sum(values: &Arr<i64>) -> i64 {
        crate::cross_call_master::CounterSum(values)
    }

}

impl std::cmp::PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl std::cmp::Eq for Counter {}
impl std::cmp::PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.handle).partial_cmp(&(other.handle))
    }
}
impl std::cmp::Ord for Counter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.handle).cmp(&(other.handle))
    }
}


