// Generated from cross_call_master.pplugin

#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use plugify::{PlgString, PlgVector, PlgVariant, Vector2, Vector3, Vector4, Matrix4x4};

pub type NoParamReturnFunctionCallbackFunc = unsafe extern "C" fn() -> i32;

pub type FuncVoid = unsafe extern "C" fn();

pub type FuncBool = unsafe extern "C" fn() -> bool;

pub type FuncChar8 = unsafe extern "C" fn() -> i8;

pub type FuncChar16 = unsafe extern "C" fn() -> u16;

pub type FuncInt8 = unsafe extern "C" fn() -> i8;

pub type FuncInt16 = unsafe extern "C" fn() -> i16;

pub type FuncInt32 = unsafe extern "C" fn() -> i32;

pub type FuncInt64 = unsafe extern "C" fn() -> i64;

pub type FuncUInt8 = unsafe extern "C" fn() -> u8;

pub type FuncUInt16 = unsafe extern "C" fn() -> u16;

pub type FuncUInt32 = unsafe extern "C" fn() -> u32;

pub type FuncUInt64 = unsafe extern "C" fn() -> u64;

pub type FuncPtr = unsafe extern "C" fn() -> usize;

pub type FuncFloat = unsafe extern "C" fn() -> f32;

pub type FuncDouble = unsafe extern "C" fn() -> f64;

pub type FuncString = unsafe extern "C" fn() -> PlgString;

pub type FuncAny = unsafe extern "C" fn() -> PlgVariant;

pub type FuncFunction = unsafe extern "C" fn() -> usize;

pub type FuncBoolVector = unsafe extern "C" fn() -> PlgVector<bool>;

pub type FuncChar8Vector = unsafe extern "C" fn() -> PlgVector<i8>;

pub type FuncChar16Vector = unsafe extern "C" fn() -> PlgVector<u16>;

pub type FuncInt8Vector = unsafe extern "C" fn() -> PlgVector<i8>;

pub type FuncInt16Vector = unsafe extern "C" fn() -> PlgVector<i16>;

pub type FuncInt32Vector = unsafe extern "C" fn() -> PlgVector<i32>;

pub type FuncInt64Vector = unsafe extern "C" fn() -> PlgVector<i64>;

pub type FuncUInt8Vector = unsafe extern "C" fn() -> PlgVector<u8>;

pub type FuncUInt16Vector = unsafe extern "C" fn() -> PlgVector<u16>;

pub type FuncUInt32Vector = unsafe extern "C" fn() -> PlgVector<u32>;

pub type FuncUInt64Vector = unsafe extern "C" fn() -> PlgVector<u64>;

pub type FuncPtrVector = unsafe extern "C" fn() -> PlgVector<usize>;

pub type FuncFloatVector = unsafe extern "C" fn() -> PlgVector<f32>;

pub type FuncDoubleVector = unsafe extern "C" fn() -> PlgVector<f64>;

pub type FuncStringVector = unsafe extern "C" fn() -> PlgVector<PlgString>;

pub type FuncAnyVector = unsafe extern "C" fn() -> PlgVector<PlgVariant>;

pub type FuncVec2Vector = unsafe extern "C" fn() -> PlgVector<Vector2>;

pub type FuncVec3Vector = unsafe extern "C" fn() -> PlgVector<Vector3>;

pub type FuncVec4Vector = unsafe extern "C" fn() -> PlgVector<Vector4>;

pub type FuncMat4x4Vector = unsafe extern "C" fn() -> PlgVector<Matrix4x4>;

pub type FuncVec2 = unsafe extern "C" fn() -> Vector2;

pub type FuncVec3 = unsafe extern "C" fn() -> Vector3;

pub type FuncVec4 = unsafe extern "C" fn() -> Vector4;

pub type FuncMat4x4 = unsafe extern "C" fn() -> Matrix4x4;

pub type Func1 = unsafe extern "C" fn(&Vector3) -> i32;

pub type Func2 = unsafe extern "C" fn(f32, i64) -> i8;

pub type Func3 = unsafe extern "C" fn(usize, &Vector4, &PlgString);

pub type Func4 = unsafe extern "C" fn(bool, i32, u16, &Matrix4x4) -> Vector4;

pub type Func5 = unsafe extern "C" fn(i8, &Vector2, usize, f64, &PlgVector<u64>) -> bool;

pub type Func6 = unsafe extern "C" fn(&PlgString, f32, &PlgVector<f32>, i16, &PlgVector<u8>, usize) -> i64;

pub type Func7 = unsafe extern "C" fn(&PlgVector<i8>, u16, u16, &PlgVector<u32>, &Vector4, bool, u64) -> f64;

pub type Func8 = unsafe extern "C" fn(&Vector3, &PlgVector<u32>, i16, bool, &Vector4, &PlgVector<u16>, u16, i32) -> Matrix4x4;

pub type Func9 = unsafe extern "C" fn(f32, &Vector2, &PlgVector<i8>, u64, bool, &PlgString, &Vector4, i16, usize);

pub type Func10 = unsafe extern "C" fn(&Vector4, &Matrix4x4, &PlgVector<u32>, u64, &PlgVector<i8>, i32, bool, &Vector2, i64, f64) -> u32;

pub type Func11 = unsafe extern "C" fn(&PlgVector<bool>, u16, u8, f64, &Vector3, &PlgVector<i8>, i64, u16, f32, &Vector2, u32) -> usize;

pub type Func12 = unsafe extern "C" fn(usize, &PlgVector<f64>, u32, f64, bool, i32, i8, u64, f32, &PlgVector<usize>, i64, i8) -> bool;

pub type Func13 = unsafe extern "C" fn(i64, &PlgVector<i8>, u16, f32, &PlgVector<bool>, &Vector4, &PlgString, i32, &Vector3, usize, &Vector2, &PlgVector<u8>, i16) -> PlgString;

pub type Func14 = unsafe extern "C" fn(&PlgVector<i8>, &PlgVector<u32>, &Matrix4x4, bool, u16, i32, &PlgVector<f32>, u16, &PlgVector<u8>, i8, &Vector3, &Vector4, f64, usize) -> PlgVector<PlgString>;

pub type Func15 = unsafe extern "C" fn(&PlgVector<i16>, &Matrix4x4, &Vector4, usize, u64, &PlgVector<u32>, bool, f32, &PlgVector<u16>, u8, i32, &Vector2, u16, f64, &PlgVector<u8>) -> i16;

pub type Func16 = unsafe extern "C" fn(&PlgVector<bool>, i16, &PlgVector<i8>, &Vector4, &Matrix4x4, &Vector2, &PlgVector<u64>, &PlgVector<i8>, &PlgString, i64, &PlgVector<u32>, &Vector3, f32, f64, i8, u16) -> usize;

pub type Func17 = unsafe extern "C" fn(&mut i32);

pub type Func18 = unsafe extern "C" fn(&mut i8, &mut i16) -> Vector2;

pub type Func19 = unsafe extern "C" fn(&mut u32, &mut Vector3, &mut PlgVector<u32>);

pub type Func20 = unsafe extern "C" fn(&mut u16, &mut Vector4, &mut PlgVector<u64>, &mut i8) -> i32;

pub type Func21 = unsafe extern "C" fn(&mut Matrix4x4, &mut PlgVector<i32>, &mut Vector2, &mut bool, &mut f64) -> f32;

pub type Func22 = unsafe extern "C" fn(&mut usize, &mut u32, &mut PlgVector<f64>, &mut i16, &mut PlgString, &mut Vector4) -> u64;

pub type Func23 = unsafe extern "C" fn(&mut u64, &mut Vector2, &mut PlgVector<i16>, &mut u16, &mut f32, &mut i8, &mut PlgVector<u8>);

pub type Func24 = unsafe extern "C" fn(&mut PlgVector<i8>, &mut i64, &mut PlgVector<u8>, &mut Vector4, &mut u64, &mut PlgVector<usize>, &mut f64, &mut PlgVector<usize>) -> Matrix4x4;

pub type Func25 = unsafe extern "C" fn(&mut i32, &mut PlgVector<usize>, &mut bool, &mut u8, &mut PlgString, &mut Vector3, &mut i64, &mut Vector4, &mut u16) -> f64;

pub type Func26 = unsafe extern "C" fn(&mut u16, &mut Vector2, &mut Matrix4x4, &mut PlgVector<f32>, &mut i16, &mut u64, &mut u32, &mut PlgVector<u16>, &mut usize, &mut bool) -> i8;

pub type Func27 = unsafe extern "C" fn(&mut f32, &mut Vector3, &mut usize, &mut Vector2, &mut PlgVector<i16>, &mut Matrix4x4, &mut bool, &mut Vector4, &mut i8, &mut i32, &mut PlgVector<u8>) -> u8;

pub type Func28 = unsafe extern "C" fn(&mut usize, &mut u16, &mut PlgVector<u32>, &mut Matrix4x4, &mut f32, &mut Vector4, &mut PlgString, &mut PlgVector<u64>, &mut i64, &mut bool, &mut Vector3, &mut PlgVector<f32>) -> PlgString;

pub type Func29 = unsafe extern "C" fn(&mut Vector4, &mut i32, &mut PlgVector<i8>, &mut f64, &mut bool, &mut i8, &mut PlgVector<u16>, &mut f32, &mut PlgString, &mut Matrix4x4, &mut u64, &mut Vector3, &mut PlgVector<i64>) -> PlgVector<PlgString>;

pub type Func30 = unsafe extern "C" fn(&mut usize, &mut Vector4, &mut i64, &mut PlgVector<u32>, &mut bool, &mut PlgString, &mut Vector3, &mut PlgVector<u8>, &mut f32, &mut Vector2, &mut Matrix4x4, &mut i8, &mut PlgVector<f32>, &mut f64) -> i32;

pub type Func31 = unsafe extern "C" fn(&mut i8, &mut u32, &mut PlgVector<u64>, &mut Vector4, &mut PlgString, &mut bool, &mut i64, &mut Vector2, &mut i8, &mut u16, &mut PlgVector<i16>, &mut Matrix4x4, &mut Vector3, &mut f32, &mut PlgVector<f64>) -> Vector3;

pub type Func32 = unsafe extern "C" fn(&mut i32, &mut u16, &mut PlgVector<i8>, &mut Vector4, &mut usize, &mut PlgVector<u32>, &mut Matrix4x4, &mut u64, &mut PlgString, &mut i64, &mut Vector2, &mut PlgVector<i8>, &mut bool, &mut Vector3, &mut u8, &mut PlgVector<u16>) -> f64;

pub type Func33 = unsafe extern "C" fn(&mut PlgVariant);

pub type FuncEnum = unsafe extern "C" fn(Example, &mut PlgVector<Example>) -> PlgVector<Example>;

