// Generated from cross_call_master.pplugin (group: resource)

#[allow(unused_imports)]
use std::sync::RwLock;
#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use super::delegates::*;
#[allow(unused_imports)]
use plugify::{get_method_ptr, Str, Arr, Var, Vec2, Vec3, Vec4, Mat4x4};

/// # Arguments
/// * `id` - (int32)
/// * `name` - (string)
///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleCreate(id: i32, name: &Str) -> usize {
    unsafe { __cross_call_master_ResourceHandleCreate.expect("ResourceHandleCreate function was not found")(id, name) }
}
pub type _ResourceHandleCreate = unsafe extern "C" fn(i32, &Str) -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleCreate: Option<_ResourceHandleCreate> = None;

///
/// # Returns
/// ptr64
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleCreateDefault() -> usize {
    unsafe { __cross_call_master_ResourceHandleCreateDefault.expect("ResourceHandleCreateDefault function was not found")() }
}
pub type _ResourceHandleCreateDefault = unsafe extern "C" fn() -> usize;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleCreateDefault: Option<_ResourceHandleCreateDefault> = None;

/// # Arguments
/// * `handle` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleDestroy(handle: usize) {
    unsafe { __cross_call_master_ResourceHandleDestroy.expect("ResourceHandleDestroy function was not found")(handle) }
}
pub type _ResourceHandleDestroy = unsafe extern "C" fn(usize);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleDestroy: Option<_ResourceHandleDestroy> = None;

/// # Arguments
/// * `handle` - (ptr64)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetId(handle: usize) -> i32 {
    unsafe { __cross_call_master_ResourceHandleGetId.expect("ResourceHandleGetId function was not found")(handle) }
}
pub type _ResourceHandleGetId = unsafe extern "C" fn(usize) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleGetId: Option<_ResourceHandleGetId> = None;

/// # Arguments
/// * `handle` - (ptr64)
///
/// # Returns
/// string
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetName(handle: usize) -> Str {
    unsafe { __cross_call_master_ResourceHandleGetName.expect("ResourceHandleGetName function was not found")(handle) }
}
pub type _ResourceHandleGetName = unsafe extern "C" fn(usize) -> Str;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleGetName: Option<_ResourceHandleGetName> = None;

/// # Arguments
/// * `handle` - (ptr64)
/// * `name` - (string)
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleSetName(handle: usize, name: &Str) {
    unsafe { __cross_call_master_ResourceHandleSetName.expect("ResourceHandleSetName function was not found")(handle, name) }
}
pub type _ResourceHandleSetName = unsafe extern "C" fn(usize, &Str);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleSetName: Option<_ResourceHandleSetName> = None;

/// # Arguments
/// * `handle` - (ptr64)
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleIncrementCounter(handle: usize) {
    unsafe { __cross_call_master_ResourceHandleIncrementCounter.expect("ResourceHandleIncrementCounter function was not found")(handle) }
}
pub type _ResourceHandleIncrementCounter = unsafe extern "C" fn(usize);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleIncrementCounter: Option<_ResourceHandleIncrementCounter> = None;

/// # Arguments
/// * `handle` - (ptr64)
///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetCounter(handle: usize) -> i32 {
    unsafe { __cross_call_master_ResourceHandleGetCounter.expect("ResourceHandleGetCounter function was not found")(handle) }
}
pub type _ResourceHandleGetCounter = unsafe extern "C" fn(usize) -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleGetCounter: Option<_ResourceHandleGetCounter> = None;

/// # Arguments
/// * `handle` - (ptr64)
/// * `value` - (float)
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleAddData(handle: usize, value: f32) {
    unsafe { __cross_call_master_ResourceHandleAddData.expect("ResourceHandleAddData function was not found")(handle, value) }
}
pub type _ResourceHandleAddData = unsafe extern "C" fn(usize, f32);
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleAddData: Option<_ResourceHandleAddData> = None;

/// # Arguments
/// * `handle` - (ptr64)
///
/// # Returns
/// float[]
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetData(handle: usize) -> Arr<f32> {
    unsafe { __cross_call_master_ResourceHandleGetData.expect("ResourceHandleGetData function was not found")(handle) }
}
pub type _ResourceHandleGetData = unsafe extern "C" fn(usize) -> Arr<f32>;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleGetData: Option<_ResourceHandleGetData> = None;

///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetAliveCount() -> i32 {
    unsafe { __cross_call_master_ResourceHandleGetAliveCount.expect("ResourceHandleGetAliveCount function was not found")() }
}
pub type _ResourceHandleGetAliveCount = unsafe extern "C" fn() -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleGetAliveCount: Option<_ResourceHandleGetAliveCount> = None;

///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetTotalCreated() -> i32 {
    unsafe { __cross_call_master_ResourceHandleGetTotalCreated.expect("ResourceHandleGetTotalCreated function was not found")() }
}
pub type _ResourceHandleGetTotalCreated = unsafe extern "C" fn() -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleGetTotalCreated: Option<_ResourceHandleGetTotalCreated> = None;

///
/// # Returns
/// int32
#[allow(dead_code, non_snake_case)]
pub fn ResourceHandleGetTotalDestroyed() -> i32 {
    unsafe { __cross_call_master_ResourceHandleGetTotalDestroyed.expect("ResourceHandleGetTotalDestroyed function was not found")() }
}
pub type _ResourceHandleGetTotalDestroyed = unsafe extern "C" fn() -> i32;
#[allow(dead_code, non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut __cross_call_master_ResourceHandleGetTotalDestroyed: Option<_ResourceHandleGetTotalDestroyed> = None;

