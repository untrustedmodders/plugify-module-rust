// Generated from cross_call_master.pplugin (group: resource)

#[allow(unused_imports)]
use std::sync::OnceLock;
#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use super::delegates::*;
#[allow(unused_imports)]
use plugify::{get_method_ptr, PlgString, PlgVector, PlgVariant, Vector2, Vector3, Vector4, Matrix4x4};

/// # Arguments
/// * `id` - (int32)
/// # Arguments
/// * `name` - (string)
///
/// # Returns
/// ptr64
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleCreate(id: i32, name: &PlgString) -> usize {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i32, &PlgString) -> usize> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleCreate";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(id, name)
    }
}

///
/// # Returns
/// ptr64
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleCreateDefault() -> usize {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> usize> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleCreateDefault";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

/// # Arguments
/// * `handle` - (ptr64)
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleDestroy(handle: usize) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleDestroy";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(handle);
    }
}

/// # Arguments
/// * `handle` - (ptr64)
///
/// # Returns
/// int32
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetId(handle: usize) -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize) -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleGetId";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(handle)
    }
}

/// # Arguments
/// * `handle` - (ptr64)
///
/// # Returns
/// string
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetName(handle: usize) -> PlgString {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize) -> PlgString> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleGetName";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(handle)
    }
}

/// # Arguments
/// * `handle` - (ptr64)
/// # Arguments
/// * `name` - (string)
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleSetName(handle: usize, name: &PlgString) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize, &PlgString)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleSetName";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(handle, name);
    }
}

/// # Arguments
/// * `handle` - (ptr64)
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleIncrementCounter(handle: usize) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleIncrementCounter";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(handle);
    }
}

/// # Arguments
/// * `handle` - (ptr64)
///
/// # Returns
/// int32
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetCounter(handle: usize) -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize) -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleGetCounter";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(handle)
    }
}

/// # Arguments
/// * `handle` - (ptr64)
/// # Arguments
/// * `value` - (float)
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleAddData(handle: usize, value: f32) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize, f32)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleAddData";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(handle, value);
    }
}

/// # Arguments
/// * `handle` - (ptr64)
///
/// # Returns
/// float[]
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetData(handle: usize) -> PlgVector<f32> {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize) -> PlgVector<f32>> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleGetData";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(handle)
    }
}

///
/// # Returns
/// int32
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetAliveCount() -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleGetAliveCount";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int32
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetTotalCreated() -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleGetTotalCreated";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

///
/// # Returns
/// int32
#[inline]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetTotalDestroyed() -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.ResourceHandleGetTotalDestroyed";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

