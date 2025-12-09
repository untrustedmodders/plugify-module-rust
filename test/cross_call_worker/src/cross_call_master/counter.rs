// Generated from cross_call_master.pplugin (group: counter)

#[allow(unused_imports)]
use std::sync::OnceLock;
#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use super::delegates::*;
#[allow(unused_imports)]
use plugify::{get_method_ptr, PlgString, PlgVector, PlgVariant, Vector2, Vector3, Vector4, Matrix4x4};

/// # Arguments
/// * `initialValue` - (int64)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CounterCreate(initialValue: i64) -> usize {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i64) -> usize> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterCreate";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(initialValue)
    }
}

///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CounterCreateZero() -> usize {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn() -> usize> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterCreateZero";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func()
    }
}

/// # Arguments
/// * `counter` - (ptr64)
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn CounterGetValue(counter: usize) -> i64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize) -> i64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterGetValue";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(counter)
    }
}

/// # Arguments
/// * `counter` - (ptr64)
/// # Arguments
/// * `value` - (int64)
#[allow(dead_code, non_snake_case)]
pub fn CounterSetValue(counter: usize, value: i64) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize, i64)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterSetValue";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(counter, value);
    }
}

/// # Arguments
/// * `counter` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn CounterIncrement(counter: usize) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterIncrement";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(counter);
    }
}

/// # Arguments
/// * `counter` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn CounterDecrement(counter: usize) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterDecrement";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(counter);
    }
}

/// # Arguments
/// * `counter` - (ptr64)
/// # Arguments
/// * `amount` - (int64)
#[allow(dead_code, non_snake_case)]
pub fn CounterAdd(counter: usize, amount: i64) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize, i64)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterAdd";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(counter, amount);
    }
}

/// # Arguments
/// * `counter` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn CounterReset(counter: usize) {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize)> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterReset";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(counter);
    }
}

/// # Arguments
/// * `counter` - (ptr64)
///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn CounterIsPositive(counter: usize) -> bool {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(usize) -> bool> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterIsPositive";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(counter)
    }
}

/// # Arguments
/// * `value1` - (int64)
/// # Arguments
/// * `value2` - (int64)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn CounterCompare(value1: i64, value2: i64) -> i32 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(i64, i64) -> i32> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterCompare";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(value1, value2)
    }
}

/// # Arguments
/// * `values` - (int64[])
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn CounterSum(values: &PlgVector<i64>) -> i64 {
    unsafe {
        static FUNC: OnceLock<unsafe extern "C" fn(&PlgVector<i64>) -> i64> = OnceLock::new();
        let __func = FUNC.get_or_init(|| {
            let name = "cross_call_master.CounterSum";
            let ptr = get_method_ptr(name.as_ptr(), name.len());
            std::mem::transmute(ptr)
        });
        __func(values)
    }
}

