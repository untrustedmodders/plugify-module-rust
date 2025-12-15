// Generated from cross_call_master.pplugin (group: counter)

#[allow(unused_imports)]
use std::sync::RwLock;
#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use super::delegates::*;
#[allow(unused_imports)]
use plugify::{get_method_ptr, Str, Arr, Var, Vec2, Vec3, Vec4, Mat4x4};

/// # Arguments
/// * `initialValue` - (int64)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CounterCreate(initialValue: i64) -> usize {
    unsafe { __cross_call_master_CounterCreate.expect("CounterCreate function was not found")(initialValue) }
}
pub type _CounterCreate = unsafe extern "C" fn(i64) -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterCreate: Option<_CounterCreate> = None;

///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn CounterCreateZero() -> usize {
    unsafe { __cross_call_master_CounterCreateZero.expect("CounterCreateZero function was not found")() }
}
pub type _CounterCreateZero = unsafe extern "C" fn() -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterCreateZero: Option<_CounterCreateZero> = None;

/// # Arguments
/// * `counter` - (ptr64)
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn CounterGetValue(counter: usize) -> i64 {
    unsafe { __cross_call_master_CounterGetValue.expect("CounterGetValue function was not found")(counter) }
}
pub type _CounterGetValue = unsafe extern "C" fn(usize) -> i64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterGetValue: Option<_CounterGetValue> = None;

/// # Arguments
/// * `counter` - (ptr64)
/// * `value` - (int64)
#[allow(dead_code, non_snake_case)]
pub fn CounterSetValue(counter: usize, value: i64) {
    unsafe { __cross_call_master_CounterSetValue.expect("CounterSetValue function was not found")(counter, value) }
}
pub type _CounterSetValue = unsafe extern "C" fn(usize, i64);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterSetValue: Option<_CounterSetValue> = None;

/// # Arguments
/// * `counter` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn CounterIncrement(counter: usize) {
    unsafe { __cross_call_master_CounterIncrement.expect("CounterIncrement function was not found")(counter) }
}
pub type _CounterIncrement = unsafe extern "C" fn(usize);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterIncrement: Option<_CounterIncrement> = None;

/// # Arguments
/// * `counter` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn CounterDecrement(counter: usize) {
    unsafe { __cross_call_master_CounterDecrement.expect("CounterDecrement function was not found")(counter) }
}
pub type _CounterDecrement = unsafe extern "C" fn(usize);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterDecrement: Option<_CounterDecrement> = None;

/// # Arguments
/// * `counter` - (ptr64)
/// * `amount` - (int64)
#[allow(dead_code, non_snake_case)]
pub fn CounterAdd(counter: usize, amount: i64) {
    unsafe { __cross_call_master_CounterAdd.expect("CounterAdd function was not found")(counter, amount) }
}
pub type _CounterAdd = unsafe extern "C" fn(usize, i64);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterAdd: Option<_CounterAdd> = None;

/// # Arguments
/// * `counter` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn CounterReset(counter: usize) {
    unsafe { __cross_call_master_CounterReset.expect("CounterReset function was not found")(counter) }
}
pub type _CounterReset = unsafe extern "C" fn(usize);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterReset: Option<_CounterReset> = None;

/// # Arguments
/// * `counter` - (ptr64)
///
/// # Returns
/// bool
#[allow(dead_code, non_snake_case)]
pub fn CounterIsPositive(counter: usize) -> bool {
    unsafe { __cross_call_master_CounterIsPositive.expect("CounterIsPositive function was not found")(counter) }
}
pub type _CounterIsPositive = unsafe extern "C" fn(usize) -> bool;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterIsPositive: Option<_CounterIsPositive> = None;

/// # Arguments
/// * `value1` - (int64)
/// * `value2` - (int64)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn CounterCompare(value1: i64, value2: i64) -> i32 {
    unsafe { __cross_call_master_CounterCompare.expect("CounterCompare function was not found")(value1, value2) }
}
pub type _CounterCompare = unsafe extern "C" fn(i64, i64) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterCompare: Option<_CounterCompare> = None;

/// # Arguments
/// * `values` - (int64[])
///
/// # Returns
/// int64
#[allow(dead_code, non_snake_case)]
pub fn CounterSum(values: &Arr<i64>) -> i64 {
    unsafe { __cross_call_master_CounterSum.expect("CounterSum function was not found")(values) }
}
pub type _CounterSum = unsafe extern "C" fn(&Arr<i64>) -> i64;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_CounterSum: Option<_CounterSum> = None;

