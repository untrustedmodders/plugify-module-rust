// Generated from cross_call_master.pplugin (group: core)

#[allow(unused_imports)]
use std::sync::OnceLock;
#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use super::delegates::*;
#[allow(unused_imports)]
use plugify::{get_method_ptr, PlgString, PlgVector, PlgVariant, Vector2, Vector3, Vector4, Matrix4x4};

/// # Arguments
/// * `returnString` - (string)
#[allow(dead_code, non_snake_case)]
pub fn ReverseReturn(returnString: &PlgString) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&PlgString)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ReverseReturn";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(returnString);
    }
}

#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnVoidCallback() {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn()> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnVoidCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func();
    }
}

///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnBoolCallback() -> bool {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> bool> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnBoolCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// char8
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnChar8Callback() -> i8 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> i8> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnChar8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// char16
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnChar16Callback() -> u16 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> u16> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnChar16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int8
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnInt8Callback() -> i8 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> i8> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnInt8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int16
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnInt16Callback() -> i16 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> i16> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnInt16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnInt32Callback() -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnInt32Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnInt64Callback() -> i64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> i64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnInt64Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// uint8
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnUInt8Callback() -> u8 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> u8> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnUInt8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// uint16
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnUInt16Callback() -> u16 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> u16> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnUInt16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// uint32
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnUInt32Callback() -> u32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> u32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnUInt32Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// uint64
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnUInt64Callback() -> u64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> u64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnUInt64Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnPointerCallback() -> usize {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> usize> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnPointerCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// float
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnFloatCallback() -> f32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> f32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnFloatCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// double
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnDoubleCallback() -> f64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> f64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnDoubleCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// function
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnFunctionCallback() -> NoParamReturnFunctionCallbackFunc {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> NoParamReturnFunctionCallbackFunc> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnFunctionCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnStringCallback() -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnStringCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// any
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnAnyCallback() -> PlgVariant {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVariant> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnAnyCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// bool[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayBoolCallback() -> PlgVector<bool> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<bool>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayBoolCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// char8[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayChar8Callback() -> PlgVector<i8> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<i8>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayChar8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// char16[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayChar16Callback() -> PlgVector<u16> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<u16>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayChar16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int8[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayInt8Callback() -> PlgVector<i8> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<i8>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayInt8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int16[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayInt16Callback() -> PlgVector<i16> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<i16>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayInt16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int32[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayInt32Callback() -> PlgVector<i32> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<i32>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayInt32Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int64[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayInt64Callback() -> PlgVector<i64> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<i64>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayInt64Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// uint8[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayUInt8Callback() -> PlgVector<u8> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<u8>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayUInt8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// uint16[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayUInt16Callback() -> PlgVector<u16> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<u16>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayUInt16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// uint32[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayUInt32Callback() -> PlgVector<u32> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<u32>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayUInt32Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// uint64[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayUInt64Callback() -> PlgVector<u64> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<u64>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayUInt64Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// ptr64[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayPointerCallback() -> PlgVector<usize> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<usize>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayPointerCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// float[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayFloatCallback() -> PlgVector<f32> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<f32>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayFloatCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// double[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayDoubleCallback() -> PlgVector<f64> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<f64>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayDoubleCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// string[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayStringCallback() -> PlgVector<PlgString> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<PlgString>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayStringCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// any[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayAnyCallback() -> PlgVector<PlgVariant> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<PlgVariant>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayAnyCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// vec2[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayVector2Callback() -> PlgVector<Vector2> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<Vector2>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayVector2Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// vec3[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayVector3Callback() -> PlgVector<Vector3> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<Vector3>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayVector3Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// vec4[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayVector4Callback() -> PlgVector<Vector4> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<Vector4>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayVector4Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// mat4x4[]
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnArrayMatrix4x4Callback() -> PlgVector<Matrix4x4> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> PlgVector<Matrix4x4>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnArrayMatrix4x4Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// vec2
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnVector2Callback() -> Vector2 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> Vector2> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnVector2Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// vec3
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnVector3Callback() -> Vector3 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> Vector3> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnVector3Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// vec4
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnVector4Callback() -> Vector4 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> Vector4> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnVector4Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// mat4x4
#[allow(dead_code, non_snake_case)]
pub fn NoParamReturnMatrix4x4Callback() -> Matrix4x4 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> Matrix4x4> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.NoParamReturnMatrix4x4Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

/// # Arguments
/// * `a` - (int32)
#[allow(dead_code, non_snake_case)]
pub fn Param1Callback(a: i32) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param1Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a);
    }
}

/// # Arguments
/// * `a` - (int32)
/// # Arguments
/// * `b` - (float)
#[allow(dead_code, non_snake_case)]
pub fn Param2Callback(a: i32, b: f32) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, f32)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param2Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b);
    }
}

/// # Arguments
/// * `a` - (int32)
/// # Arguments
/// * `b` - (float)
/// # Arguments
/// * `c` - (double)
#[allow(dead_code, non_snake_case)]
pub fn Param3Callback(a: i32, b: f32, c: f64) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, f32, f64)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param3Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c);
    }
}

/// # Arguments
/// * `a` - (int32)
/// # Arguments
/// * `b` - (float)
/// # Arguments
/// * `c` - (double)
/// # Arguments
/// * `d` - (vec4)
#[allow(dead_code, non_snake_case)]
pub fn Param4Callback(a: i32, b: f32, c: f64, d: &Vector4) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, f32, f64, &Vector4)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param4Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d);
    }
}

/// # Arguments
/// * `a` - (int32)
/// # Arguments
/// * `b` - (float)
/// # Arguments
/// * `c` - (double)
/// # Arguments
/// * `d` - (vec4)
/// # Arguments
/// * `e` - (int64[])
#[allow(dead_code, non_snake_case)]
pub fn Param5Callback(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, f32, f64, &Vector4, &PlgVector<i64>)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param5Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e);
    }
}

/// # Arguments
/// * `a` - (int32)
/// # Arguments
/// * `b` - (float)
/// # Arguments
/// * `c` - (double)
/// # Arguments
/// * `d` - (vec4)
/// # Arguments
/// * `e` - (int64[])
/// # Arguments
/// * `f` - (char8)
#[allow(dead_code, non_snake_case)]
pub fn Param6Callback(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, f32, f64, &Vector4, &PlgVector<i64>, i8)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param6Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f);
    }
}

/// # Arguments
/// * `a` - (int32)
/// # Arguments
/// * `b` - (float)
/// # Arguments
/// * `c` - (double)
/// # Arguments
/// * `d` - (vec4)
/// # Arguments
/// * `e` - (int64[])
/// # Arguments
/// * `f` - (char8)
/// # Arguments
/// * `g` - (string)
#[allow(dead_code, non_snake_case)]
pub fn Param7Callback(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8, g: &PlgString) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, f32, f64, &Vector4, &PlgVector<i64>, i8, &PlgString)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param7Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f, g);
    }
}

/// # Arguments
/// * `a` - (int32)
/// # Arguments
/// * `b` - (float)
/// # Arguments
/// * `c` - (double)
/// # Arguments
/// * `d` - (vec4)
/// # Arguments
/// * `e` - (int64[])
/// # Arguments
/// * `f` - (char8)
/// # Arguments
/// * `g` - (string)
/// # Arguments
/// * `h` - (char16)
#[allow(dead_code, non_snake_case)]
pub fn Param8Callback(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8, g: &PlgString, h: u16) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, f32, f64, &Vector4, &PlgVector<i64>, i8, &PlgString, u16)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f, g, h);
    }
}

/// # Arguments
/// * `a` - (int32)
/// # Arguments
/// * `b` - (float)
/// # Arguments
/// * `c` - (double)
/// # Arguments
/// * `d` - (vec4)
/// # Arguments
/// * `e` - (int64[])
/// # Arguments
/// * `f` - (char8)
/// # Arguments
/// * `g` - (string)
/// # Arguments
/// * `h` - (char16)
/// # Arguments
/// * `k` - (int16)
#[allow(dead_code, non_snake_case)]
pub fn Param9Callback(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8, g: &PlgString, h: u16, k: i16) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, f32, f64, &Vector4, &PlgVector<i64>, i8, &PlgString, u16, i16)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param9Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f, g, h, k);
    }
}

/// # Arguments
/// * `a` - (int32)
/// # Arguments
/// * `b` - (float)
/// # Arguments
/// * `c` - (double)
/// # Arguments
/// * `d` - (vec4)
/// # Arguments
/// * `e` - (int64[])
/// # Arguments
/// * `f` - (char8)
/// # Arguments
/// * `g` - (string)
/// # Arguments
/// * `h` - (char16)
/// # Arguments
/// * `k` - (int16)
/// # Arguments
/// * `l` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn Param10Callback(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8, g: &PlgString, h: u16, k: i16, l: usize) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, f32, f64, &Vector4, &PlgVector<i64>, i8, &PlgString, u16, i16, usize)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.Param10Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f, g, h, k, l);
    }
}

/// # Arguments
/// * `a` - (int32&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef1Callback(a: &mut i32) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef1Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a);
    }
}

/// # Arguments
/// * `a` - (int32&)
/// # Arguments
/// * `b` - (float&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef2Callback(a: &mut i32, b: &mut f32) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32, &mut f32)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef2Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b);
    }
}

/// # Arguments
/// * `a` - (int32&)
/// # Arguments
/// * `b` - (float&)
/// # Arguments
/// * `c` - (double&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef3Callback(a: &mut i32, b: &mut f32, c: &mut f64) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32, &mut f32, &mut f64)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef3Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c);
    }
}

/// # Arguments
/// * `a` - (int32&)
/// # Arguments
/// * `b` - (float&)
/// # Arguments
/// * `c` - (double&)
/// # Arguments
/// * `d` - (vec4&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef4Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vector4)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef4Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d);
    }
}

/// # Arguments
/// * `a` - (int32&)
/// # Arguments
/// * `b` - (float&)
/// # Arguments
/// * `c` - (double&)
/// # Arguments
/// * `d` - (vec4&)
/// # Arguments
/// * `e` - (int64[]&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef5Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vector4, &mut PlgVector<i64>)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef5Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e);
    }
}

/// # Arguments
/// * `a` - (int32&)
/// # Arguments
/// * `b` - (float&)
/// # Arguments
/// * `c` - (double&)
/// # Arguments
/// * `d` - (vec4&)
/// # Arguments
/// * `e` - (int64[]&)
/// # Arguments
/// * `f` - (char8&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef6Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vector4, &mut PlgVector<i64>, &mut i8)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef6Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f);
    }
}

/// # Arguments
/// * `a` - (int32&)
/// # Arguments
/// * `b` - (float&)
/// # Arguments
/// * `c` - (double&)
/// # Arguments
/// * `d` - (vec4&)
/// # Arguments
/// * `e` - (int64[]&)
/// # Arguments
/// * `f` - (char8&)
/// # Arguments
/// * `g` - (string&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef7Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8, g: &mut PlgString) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vector4, &mut PlgVector<i64>, &mut i8, &mut PlgString)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef7Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f, g);
    }
}

/// # Arguments
/// * `a` - (int32&)
/// # Arguments
/// * `b` - (float&)
/// # Arguments
/// * `c` - (double&)
/// # Arguments
/// * `d` - (vec4&)
/// # Arguments
/// * `e` - (int64[]&)
/// # Arguments
/// * `f` - (char8&)
/// # Arguments
/// * `g` - (string&)
/// # Arguments
/// * `h` - (char16&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef8Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8, g: &mut PlgString, h: &mut u16) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vector4, &mut PlgVector<i64>, &mut i8, &mut PlgString, &mut u16)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f, g, h);
    }
}

/// # Arguments
/// * `a` - (int32&)
/// # Arguments
/// * `b` - (float&)
/// # Arguments
/// * `c` - (double&)
/// # Arguments
/// * `d` - (vec4&)
/// # Arguments
/// * `e` - (int64[]&)
/// # Arguments
/// * `f` - (char8&)
/// # Arguments
/// * `g` - (string&)
/// # Arguments
/// * `h` - (char16&)
/// # Arguments
/// * `k` - (int16&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef9Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8, g: &mut PlgString, h: &mut u16, k: &mut i16) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vector4, &mut PlgVector<i64>, &mut i8, &mut PlgString, &mut u16, &mut i16)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef9Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f, g, h, k);
    }
}

/// # Arguments
/// * `a` - (int32&)
/// # Arguments
/// * `b` - (float&)
/// # Arguments
/// * `c` - (double&)
/// # Arguments
/// * `d` - (vec4&)
/// # Arguments
/// * `e` - (int64[]&)
/// # Arguments
/// * `f` - (char8&)
/// # Arguments
/// * `g` - (string&)
/// # Arguments
/// * `h` - (char16&)
/// # Arguments
/// * `k` - (int16&)
/// # Arguments
/// * `l` - (ptr64&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRef10Callback(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8, g: &mut PlgString, h: &mut u16, k: &mut i16, l: &mut usize) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut i32, &mut f32, &mut f64, &mut Vector4, &mut PlgVector<i64>, &mut i8, &mut PlgString, &mut u16, &mut i16, &mut usize)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRef10Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(a, b, c, d, e, f, g, h, k, l);
    }
}

/// # Arguments
/// * `p1` - (bool[]&)
/// # Arguments
/// * `p2` - (char8[]&)
/// # Arguments
/// * `p3` - (char16[]&)
/// # Arguments
/// * `p4` - (int8[]&)
/// # Arguments
/// * `p5` - (int16[]&)
/// # Arguments
/// * `p6` - (int32[]&)
/// # Arguments
/// * `p7` - (int64[]&)
/// # Arguments
/// * `p8` - (uint8[]&)
/// # Arguments
/// * `p9` - (uint16[]&)
/// # Arguments
/// * `p10` - (uint32[]&)
/// # Arguments
/// * `p11` - (uint64[]&)
/// # Arguments
/// * `p12` - (ptr64[]&)
/// # Arguments
/// * `p13` - (float[]&)
/// # Arguments
/// * `p14` - (double[]&)
/// # Arguments
/// * `p15` - (string[]&)
#[allow(dead_code, non_snake_case)]
pub fn ParamRefVectorsCallback(p1: &mut PlgVector<bool>, p2: &mut PlgVector<i8>, p3: &mut PlgVector<u16>, p4: &mut PlgVector<i8>, p5: &mut PlgVector<i16>, p6: &mut PlgVector<i32>, p7: &mut PlgVector<i64>, p8: &mut PlgVector<u8>, p9: &mut PlgVector<u16>, p10: &mut PlgVector<u32>, p11: &mut PlgVector<u64>, p12: &mut PlgVector<usize>, p13: &mut PlgVector<f32>, p14: &mut PlgVector<f64>, p15: &mut PlgVector<PlgString>) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut PlgVector<bool>, &mut PlgVector<i8>, &mut PlgVector<u16>, &mut PlgVector<i8>, &mut PlgVector<i16>, &mut PlgVector<i32>, &mut PlgVector<i64>, &mut PlgVector<u8>, &mut PlgVector<u16>, &mut PlgVector<u32>, &mut PlgVector<u64>, &mut PlgVector<usize>, &mut PlgVector<f32>, &mut PlgVector<f64>, &mut PlgVector<PlgString>)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamRefVectorsCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15);
    }
}

/// # Arguments
/// * `p1` - (bool)
/// # Arguments
/// * `p2` - (char8)
/// # Arguments
/// * `p3` - (char16)
/// # Arguments
/// * `p4` - (int8)
/// # Arguments
/// * `p5` - (int16)
/// # Arguments
/// * `p6` - (int32)
/// # Arguments
/// * `p7` - (int64)
/// # Arguments
/// * `p8` - (uint8)
/// # Arguments
/// * `p9` - (uint16)
/// # Arguments
/// * `p10` - (uint32)
/// # Arguments
/// * `p11` - (uint64)
/// # Arguments
/// * `p12` - (ptr64)
/// # Arguments
/// * `p13` - (float)
/// # Arguments
/// * `p14` - (double)
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn ParamAllPrimitivesCallback(p1: bool, p2: i8, p3: u16, p4: i8, p5: i16, p6: i32, p7: i64, p8: u8, p9: u16, p10: u32, p11: u64, p12: usize, p13: f32, p14: f64) -> i64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(bool, i8, u16, i8, i16, i32, i64, u8, u16, u32, u64, usize, f32, f64) -> i64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamAllPrimitivesCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14)
    }
}

/// # Arguments
/// * `p1` - (int32)
/// # Arguments
/// * `p2` - (int32[])
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn ParamEnumCallback(p1: Example, p2: &PlgVector<Example>) -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Example, &PlgVector<Example>) -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamEnumCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(p1, p2)
    }
}

/// # Arguments
/// * `p1` - (int32&)
/// # Arguments
/// * `p2` - (int32[]&)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn ParamEnumRefCallback(p1: &mut Example, p2: &mut PlgVector<Example>) -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut Example, &mut PlgVector<Example>) -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamEnumRefCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(p1, p2)
    }
}

/// # Arguments
/// * `p1` - (any)
/// # Arguments
/// * `p2` - (any[])
#[allow(dead_code, non_snake_case)]
pub fn ParamVariantCallback(p1: &PlgVariant, p2: &PlgVector<PlgVariant>) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&PlgVariant, &PlgVector<PlgVariant>)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamVariantCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(p1, p2);
    }
}

/// # Arguments
/// * `p1` - (any&)
/// # Arguments
/// * `p2` - (any[]&)
#[allow(dead_code, non_snake_case)]
pub fn ParamVariantRefCallback(p1: &mut PlgVariant, p2: &mut PlgVector<PlgVariant>) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&mut PlgVariant, &mut PlgVector<PlgVariant>)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ParamVariantRefCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(p1, p2);
    }
}

/// # Arguments
/// * `func` - (function)
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVoidCallback(func: FuncVoid) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncVoid)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncVoidCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func);
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn CallFuncBoolCallback(func: FuncBool) -> bool {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncBool) -> bool> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncBoolCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char8
#[allow(dead_code, non_snake_case)]
pub fn CallFuncChar8Callback(func: FuncChar8) -> i8 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncChar8) -> i8> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncChar8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char16
#[allow(dead_code, non_snake_case)]
pub fn CallFuncChar16Callback(func: FuncChar16) -> u16 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncChar16) -> u16> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncChar16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int8
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt8Callback(func: FuncInt8) -> i8 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncInt8) -> i8> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncInt8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int16
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt16Callback(func: FuncInt16) -> i16 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncInt16) -> i16> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncInt16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt32Callback(func: FuncInt32) -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncInt32) -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncInt32Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt64Callback(func: FuncInt64) -> i64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncInt64) -> i64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncInt64Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint8
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt8Callback(func: FuncUInt8) -> u8 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncUInt8) -> u8> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncUInt8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint16
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt16Callback(func: FuncUInt16) -> u16 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncUInt16) -> u16> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncUInt16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint32
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt32Callback(func: FuncUInt32) -> u32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncUInt32) -> u32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncUInt32Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint64
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt64Callback(func: FuncUInt64) -> u64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncUInt64) -> u64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncUInt64Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CallFuncPtrCallback(func: FuncPtr) -> usize {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncPtr) -> usize> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncPtrCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// float
#[allow(dead_code, non_snake_case)]
pub fn CallFuncFloatCallback(func: FuncFloat) -> f32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncFloat) -> f32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncFloatCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// double
#[allow(dead_code, non_snake_case)]
pub fn CallFuncDoubleCallback(func: FuncDouble) -> f64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncDouble) -> f64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncDoubleCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFuncStringCallback(func: FuncString) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncString) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncStringCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// any
#[allow(dead_code, non_snake_case)]
pub fn CallFuncAnyCallback(func: FuncAny) -> PlgVariant {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncAny) -> PlgVariant> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncAnyCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CallFuncFunctionCallback(func: FuncFunction) -> usize {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncFunction) -> usize> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncFunctionCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// bool[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncBoolVectorCallback(func: FuncBoolVector) -> PlgVector<bool> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncBoolVector) -> PlgVector<bool>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncBoolVectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char8[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncChar8VectorCallback(func: FuncChar8Vector) -> PlgVector<i8> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncChar8Vector) -> PlgVector<i8>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncChar8VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char16[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncChar16VectorCallback(func: FuncChar16Vector) -> PlgVector<u16> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncChar16Vector) -> PlgVector<u16>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncChar16VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int8[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt8VectorCallback(func: FuncInt8Vector) -> PlgVector<i8> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncInt8Vector) -> PlgVector<i8>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncInt8VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int16[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt16VectorCallback(func: FuncInt16Vector) -> PlgVector<i16> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncInt16Vector) -> PlgVector<i16>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncInt16VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int32[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt32VectorCallback(func: FuncInt32Vector) -> PlgVector<i32> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncInt32Vector) -> PlgVector<i32>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncInt32VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int64[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncInt64VectorCallback(func: FuncInt64Vector) -> PlgVector<i64> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncInt64Vector) -> PlgVector<i64>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncInt64VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint8[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt8VectorCallback(func: FuncUInt8Vector) -> PlgVector<u8> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncUInt8Vector) -> PlgVector<u8>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncUInt8VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint16[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt16VectorCallback(func: FuncUInt16Vector) -> PlgVector<u16> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncUInt16Vector) -> PlgVector<u16>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncUInt16VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint32[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt32VectorCallback(func: FuncUInt32Vector) -> PlgVector<u32> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncUInt32Vector) -> PlgVector<u32>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncUInt32VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint64[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncUInt64VectorCallback(func: FuncUInt64Vector) -> PlgVector<u64> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncUInt64Vector) -> PlgVector<u64>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncUInt64VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncPtrVectorCallback(func: FuncPtrVector) -> PlgVector<usize> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncPtrVector) -> PlgVector<usize>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncPtrVectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// float[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncFloatVectorCallback(func: FuncFloatVector) -> PlgVector<f32> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncFloatVector) -> PlgVector<f32>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncFloatVectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// double[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncDoubleVectorCallback(func: FuncDoubleVector) -> PlgVector<f64> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncDoubleVector) -> PlgVector<f64>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncDoubleVectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncStringVectorCallback(func: FuncStringVector) -> PlgVector<PlgString> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncStringVector) -> PlgVector<PlgString>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncStringVectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// any[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncAnyVectorCallback(func: FuncAnyVector) -> PlgVector<PlgVariant> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncAnyVector) -> PlgVector<PlgVariant>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncAnyVectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec2[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec2VectorCallback(func: FuncVec2Vector) -> PlgVector<Vector2> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncVec2Vector) -> PlgVector<Vector2>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncVec2VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec3[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec3VectorCallback(func: FuncVec3Vector) -> PlgVector<Vector3> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncVec3Vector) -> PlgVector<Vector3>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncVec3VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec4[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec4VectorCallback(func: FuncVec4Vector) -> PlgVector<Vector4> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncVec4Vector) -> PlgVector<Vector4>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncVec4VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// mat4x4[]
#[allow(dead_code, non_snake_case)]
pub fn CallFuncMat4x4VectorCallback(func: FuncMat4x4Vector) -> PlgVector<Matrix4x4> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncMat4x4Vector) -> PlgVector<Matrix4x4>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncMat4x4VectorCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec2
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec2Callback(func: FuncVec2) -> Vector2 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncVec2) -> Vector2> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncVec2Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec3
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec3Callback(func: FuncVec3) -> Vector3 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncVec3) -> Vector3> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncVec3Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec4
#[allow(dead_code, non_snake_case)]
pub fn CallFuncVec4Callback(func: FuncVec4) -> Vector4 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncVec4) -> Vector4> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncVec4Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// mat4x4
#[allow(dead_code, non_snake_case)]
pub fn CallFuncMat4x4Callback(func: FuncMat4x4) -> Matrix4x4 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncMat4x4) -> Matrix4x4> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncMat4x4Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn CallFunc1Callback(func: Func1) -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func1) -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc1Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// char8
#[allow(dead_code, non_snake_case)]
pub fn CallFunc2Callback(func: Func2) -> i8 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func2) -> i8> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc2Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
#[allow(dead_code, non_snake_case)]
pub fn CallFunc3Callback(func: Func3) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func3)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc3Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func);
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// vec4
#[allow(dead_code, non_snake_case)]
pub fn CallFunc4Callback(func: Func4) -> Vector4 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func4) -> Vector4> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc4Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn CallFunc5Callback(func: Func5) -> bool {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func5) -> bool> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc5Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn CallFunc6Callback(func: Func6) -> i64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func6) -> i64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc6Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// double
#[allow(dead_code, non_snake_case)]
pub fn CallFunc7Callback(func: Func7) -> f64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func7) -> f64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc7Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// mat4x4
#[allow(dead_code, non_snake_case)]
pub fn CallFunc8Callback(func: Func8) -> Matrix4x4 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func8) -> Matrix4x4> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc8Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
#[allow(dead_code, non_snake_case)]
pub fn CallFunc9Callback(func: Func9) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func9)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc9Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func);
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// uint32
#[allow(dead_code, non_snake_case)]
pub fn CallFunc10Callback(func: Func10) -> u32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func10) -> u32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc10Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CallFunc11Callback(func: Func11) -> usize {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func11) -> usize> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc11Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn CallFunc12Callback(func: Func12) -> bool {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func12) -> bool> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc12Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc13Callback(func: Func13) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func13) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc13Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string[]
#[allow(dead_code, non_snake_case)]
pub fn CallFunc14Callback(func: Func14) -> PlgVector<PlgString> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func14) -> PlgVector<PlgString>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc14Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// int16
#[allow(dead_code, non_snake_case)]
pub fn CallFunc15Callback(func: Func15) -> i16 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func15) -> i16> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc15Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CallFunc16Callback(func: Func16) -> usize {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func16) -> usize> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc16Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc17Callback(func: Func17) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func17) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc17Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc18Callback(func: Func18) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func18) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc18Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc19Callback(func: Func19) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func19) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc19Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc20Callback(func: Func20) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func20) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc20Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc21Callback(func: Func21) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func21) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc21Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc22Callback(func: Func22) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func22) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc22Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc23Callback(func: Func23) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func23) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc23Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc24Callback(func: Func24) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func24) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc24Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc25Callback(func: Func25) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func25) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc25Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc26Callback(func: Func26) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func26) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc26Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc27Callback(func: Func27) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func27) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc27Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc28Callback(func: Func28) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func28) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc28Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc29Callback(func: Func29) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func29) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc29Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc30Callback(func: Func30) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func30) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc30Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc31Callback(func: Func31) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func31) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc31Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc32Callback(func: Func32) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func32) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc32Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFunc33Callback(func: Func33) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(Func33) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFunc33Callback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

/// # Arguments
/// * `func` - (function)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn CallFuncEnumCallback(func: FuncEnum) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(FuncEnum) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CallFuncEnumCallback";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(func)
    }
}

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
    #[allow(dead_code, non_snake_case)]
    pub fn new_ResourceHandleCreate(id: i32, name: &PlgString) -> Result<Self, ResourceHandleError> {
        let h = crate::cross_call_master::ResourceHandleCreate(id, name);
        if h == Default::default() {
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
        if h == Default::default() {
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
        self.handle = Default::default();
        self.ownership = Ownership::Borrowed;
        h
    }

    /// Destroys and resets the handle
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        if self.handle != Default::default() && self.ownership == Ownership::Owned {
            crate::cross_call_master::ResourceHandleDestroy(self.handle);
        }
        self.handle = Default::default();
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
        self.handle != Default::default()
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetId(&self) -> Result<i32, ResourceHandleError> {
        if self.handle == Default::default() {
            return Err(ResourceHandleError::EmptyHandle);
        }
        Ok(crate::cross_call_master::ResourceHandleGetId(self.handle))
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetName(&self) -> Result<PlgString, ResourceHandleError> {
        if self.handle == Default::default() {
            return Err(ResourceHandleError::EmptyHandle);
        }
        Ok(crate::cross_call_master::ResourceHandleGetName(self.handle))
    }

    #[allow(dead_code, non_snake_case)]
    pub fn SetName(&self, name: &PlgString) -> Result<(), ResourceHandleError> {
        if self.handle == Default::default() {
            return Err(ResourceHandleError::EmptyHandle);
        }
        crate::cross_call_master::ResourceHandleSetName(self.handle, name);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn IncrementCounter(&self) -> Result<(), ResourceHandleError> {
        if self.handle == Default::default() {
            return Err(ResourceHandleError::EmptyHandle);
        }
        crate::cross_call_master::ResourceHandleIncrementCounter(self.handle);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetCounter(&self) -> Result<i32, ResourceHandleError> {
        if self.handle == Default::default() {
            return Err(ResourceHandleError::EmptyHandle);
        }
        Ok(crate::cross_call_master::ResourceHandleGetCounter(self.handle))
    }

    #[allow(dead_code, non_snake_case)]
    pub fn AddData(&self, value: f32) -> Result<(), ResourceHandleError> {
        if self.handle == Default::default() {
            return Err(ResourceHandleError::EmptyHandle);
        }
        crate::cross_call_master::ResourceHandleAddData(self.handle, value);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetData(&self) -> Result<PlgVector<f32>, ResourceHandleError> {
        if self.handle == Default::default() {
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
        if self.handle != Default::default() && self.ownership == Ownership::Owned {
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
    #[allow(dead_code, non_snake_case)]
    pub fn new_CounterCreate(initialValue: i64) -> Result<Self, CounterError> {
        let h = crate::cross_call_master::CounterCreate(initialValue);
        if h == Default::default() {
            return Err(CounterError::EmptyHandle);
        }
        Ok(Self {
            handle: h,
        })
    }

    #[allow(dead_code, non_snake_case)]
    pub fn new_CounterCreateZero() -> Result<Self, CounterError> {
        let h = crate::cross_call_master::CounterCreateZero();
        if h == Default::default() {
            return Err(CounterError::EmptyHandle);
        }
        Ok(Self {
            handle: h,
        })
    }

    /// Construct from raw handle (does not assume ownership)rlying handle
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
        self.handle = Default::default();
        h
    }

    /// Destroys and resets the handle
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.handle = Default::default();
    }

    /// Swaps two Counter instances
    #[allow(dead_code)]
    pub fn swap(&mut self, other: &mut Counter) {
        std::mem::swap(&mut self.handle, &mut other.handle);
    }

    /// Returns true if handle is valid (not empty)
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        self.handle != Default::default()
    }

    #[allow(dead_code, non_snake_case)]
    pub fn GetValue(&self) -> Result<i64, CounterError> {
        if self.handle == Default::default() {
            return Err(CounterError::EmptyHandle);
        }
        Ok(crate::cross_call_master::CounterGetValue(self.handle))
    }

    #[allow(dead_code, non_snake_case)]
    pub fn SetValue(&self, value: i64) -> Result<(), CounterError> {
        if self.handle == Default::default() {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterSetValue(self.handle, value);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn Increment(&self) -> Result<(), CounterError> {
        if self.handle == Default::default() {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterIncrement(self.handle);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn Decrement(&self) -> Result<(), CounterError> {
        if self.handle == Default::default() {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterDecrement(self.handle);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn Add(&self, amount: i64) -> Result<(), CounterError> {
        if self.handle == Default::default() {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterAdd(self.handle, amount);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn Reset(&self) -> Result<(), CounterError> {
        if self.handle == Default::default() {
            return Err(CounterError::EmptyHandle);
        }
        crate::cross_call_master::CounterReset(self.handle);
        Ok(())
    }

    #[allow(dead_code, non_snake_case)]
    pub fn IsPositive(&self) -> Result<bool, CounterError> {
        if self.handle == Default::default() {
            return Err(CounterError::EmptyHandle);
        }
        Ok(crate::cross_call_master::CounterIsPositive(self.handle))
    }

    #[allow(dead_code, non_snake_case)]
    pub fn Compare(value1: i64, value2: i64) -> i32 {
        crate::cross_call_master::CounterCompare(value1, value2)
    }

    #[allow(dead_code, non_snake_case)]
    pub fn Sum(values: &PlgVector<i64>) -> i64 {
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


