// Generated from cross_call_master.pplugin

#[allow(unused_imports)]
use super::enums::*;
#[allow(unused_imports)]
use super::aliases::*;
#[allow(unused_imports)]
use plugify::{Str, Arr, Var, Vec2, Vec3, Vec4, Mat4x4};

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


pub type FuncString = unsafe extern "C" fn() -> Str;


pub type FuncAny = unsafe extern "C" fn() -> Var;


pub type FuncFunction = unsafe extern "C" fn() -> FuncFunctionInner;


pub type FuncFunctionInner = unsafe extern "C" fn();


pub type FuncBoolVector = unsafe extern "C" fn() -> Arr<bool>;


pub type FuncChar8Vector = unsafe extern "C" fn() -> Arr<i8>;


pub type FuncChar16Vector = unsafe extern "C" fn() -> Arr<u16>;


pub type FuncInt8Vector = unsafe extern "C" fn() -> Arr<i8>;


pub type FuncInt16Vector = unsafe extern "C" fn() -> Arr<i16>;


pub type FuncInt32Vector = unsafe extern "C" fn() -> Arr<i32>;


pub type FuncInt64Vector = unsafe extern "C" fn() -> Arr<i64>;


pub type FuncUInt8Vector = unsafe extern "C" fn() -> Arr<u8>;


pub type FuncUInt16Vector = unsafe extern "C" fn() -> Arr<u16>;


pub type FuncUInt32Vector = unsafe extern "C" fn() -> Arr<u32>;


pub type FuncUInt64Vector = unsafe extern "C" fn() -> Arr<u64>;


pub type FuncPtrVector = unsafe extern "C" fn() -> Arr<usize>;


pub type FuncFloatVector = unsafe extern "C" fn() -> Arr<f32>;


pub type FuncDoubleVector = unsafe extern "C" fn() -> Arr<f64>;


pub type FuncStringVector = unsafe extern "C" fn() -> Arr<Str>;


pub type FuncAnyVector = unsafe extern "C" fn() -> Arr<Var>;


pub type FuncVec2Vector = unsafe extern "C" fn() -> Arr<Vec2>;


pub type FuncVec3Vector = unsafe extern "C" fn() -> Arr<Vec3>;


pub type FuncVec4Vector = unsafe extern "C" fn() -> Arr<Vec4>;


pub type FuncMat4x4Vector = unsafe extern "C" fn() -> Arr<Mat4x4>;


pub type FuncVec2 = unsafe extern "C" fn() -> Vec2;


pub type FuncVec3 = unsafe extern "C" fn() -> Vec3;


pub type FuncVec4 = unsafe extern "C" fn() -> Vec4;


pub type FuncMat4x4 = unsafe extern "C" fn() -> Mat4x4;


pub type FuncAliasBool = unsafe extern "C" fn() -> AliasBool;


pub type FuncAliasChar8 = unsafe extern "C" fn() -> AliasChar8;


pub type FuncAliasChar16 = unsafe extern "C" fn() -> AliasChar16;


pub type FuncAliasInt8 = unsafe extern "C" fn() -> AliasInt8;


pub type FuncAliasInt16 = unsafe extern "C" fn() -> AliasInt16;


pub type FuncAliasInt32 = unsafe extern "C" fn() -> AliasInt32;


pub type FuncAliasInt64 = unsafe extern "C" fn() -> AliasInt64;


pub type FuncAliasUInt8 = unsafe extern "C" fn() -> AliasUInt8;


pub type FuncAliasUInt16 = unsafe extern "C" fn() -> AliasUInt16;


pub type FuncAliasUInt32 = unsafe extern "C" fn() -> AliasUInt32;


pub type FuncAliasUInt64 = unsafe extern "C" fn() -> AliasUInt64;


pub type FuncAliasPtr = unsafe extern "C" fn() -> AliasPtr;


pub type FuncAliasFloat = unsafe extern "C" fn() -> AliasFloat;


pub type FuncAliasDouble = unsafe extern "C" fn() -> AliasDouble;


pub type FuncAliasString = unsafe extern "C" fn() -> AliasString;


pub type FuncAliasAny = unsafe extern "C" fn() -> AliasAny;


pub type FuncAliasFunction = unsafe extern "C" fn() -> AliasFunction;


#[allow(dead_code)]
pub type FuncAliasFunctionInner = unsafe extern "C" fn();


pub type FuncAliasBoolVector = unsafe extern "C" fn() -> AliasBoolVector;


pub type FuncAliasChar8Vector = unsafe extern "C" fn() -> AliasChar8Vector;


pub type FuncAliasChar16Vector = unsafe extern "C" fn() -> AliasChar16Vector;


pub type FuncAliasInt8Vector = unsafe extern "C" fn() -> AliasInt8Vector;


pub type FuncAliasInt16Vector = unsafe extern "C" fn() -> AliasInt16Vector;


pub type FuncAliasInt32Vector = unsafe extern "C" fn() -> AliasInt32Vector;


pub type FuncAliasInt64Vector = unsafe extern "C" fn() -> AliasInt64Vector;


pub type FuncAliasUInt8Vector = unsafe extern "C" fn() -> AliasUInt8Vector;


pub type FuncAliasUInt16Vector = unsafe extern "C" fn() -> AliasUInt16Vector;


pub type FuncAliasUInt32Vector = unsafe extern "C" fn() -> AliasUInt32Vector;


pub type FuncAliasUInt64Vector = unsafe extern "C" fn() -> AliasUInt64Vector;


pub type FuncAliasPtrVector = unsafe extern "C" fn() -> AliasPtrVector;


pub type FuncAliasFloatVector = unsafe extern "C" fn() -> AliasFloatVector;


pub type FuncAliasDoubleVector = unsafe extern "C" fn() -> AliasDoubleVector;


pub type FuncAliasStringVector = unsafe extern "C" fn() -> AliasStringVector;


pub type FuncAliasAnyVector = unsafe extern "C" fn() -> AliasAnyVector;


pub type FuncAliasVec2Vector = unsafe extern "C" fn() -> AliasVec2Vector;


pub type FuncAliasVec3Vector = unsafe extern "C" fn() -> AliasVec3Vector;


pub type FuncAliasVec4Vector = unsafe extern "C" fn() -> AliasVec4Vector;


pub type FuncAliasMat4x4Vector = unsafe extern "C" fn() -> AliasMat4x4Vector;


pub type FuncAliasVec2 = unsafe extern "C" fn() -> AliasVec2;


pub type FuncAliasVec3 = unsafe extern "C" fn() -> AliasVec3;


pub type FuncAliasVec4 = unsafe extern "C" fn() -> AliasVec4;


pub type FuncAliasMat4x4 = unsafe extern "C" fn() -> AliasMat4x4;


pub type FuncAliasAll = unsafe extern "C" fn(AliasBool, AliasChar8, AliasChar16, AliasInt8, AliasInt16, AliasInt32, AliasInt64, AliasPtr, AliasFloat, AliasDouble, &AliasString, &AliasAny, &AliasVec2, &AliasVec3, &AliasVec4, &AliasMat4x4, &AliasBoolVector, &AliasChar8Vector, &AliasChar16Vector, &AliasInt8Vector, &AliasInt16Vector, &AliasInt32Vector, &AliasInt64Vector, &AliasPtrVector, &AliasFloatVector, &AliasDoubleVector, &AliasStringVector, &AliasAnyVector, &AliasVec2Vector, &AliasVec3Vector, &AliasVec4Vector) -> Str;


pub type Func1 = unsafe extern "C" fn(&Vec3) -> i32;


pub type Func2 = unsafe extern "C" fn(f32, i64) -> i8;


pub type Func3 = unsafe extern "C" fn(usize, &Vec4, &Str);


pub type Func4 = unsafe extern "C" fn(bool, i32, u16, &Mat4x4) -> Vec4;


pub type Func5 = unsafe extern "C" fn(i8, &Vec2, usize, f64, &Arr<u64>) -> bool;


pub type Func6 = unsafe extern "C" fn(&Str, f32, &Arr<f32>, i16, &Arr<u8>, usize) -> i64;


pub type Func7 = unsafe extern "C" fn(&Arr<i8>, u16, u16, &Arr<u32>, &Vec4, bool, u64) -> f64;


pub type Func8 = unsafe extern "C" fn(&Vec3, &Arr<u32>, i16, bool, &Vec4, &Arr<u16>, u16, i32) -> Mat4x4;


pub type Func9 = unsafe extern "C" fn(f32, &Vec2, &Arr<i8>, u64, bool, &Str, &Vec4, i16, usize);


pub type Func10 = unsafe extern "C" fn(&Vec4, &Mat4x4, &Arr<u32>, u64, &Arr<i8>, i32, bool, &Vec2, i64, f64) -> u32;


pub type Func11 = unsafe extern "C" fn(&Arr<bool>, u16, u8, f64, &Vec3, &Arr<i8>, i64, u16, f32, &Vec2, u32) -> usize;


pub type Func12 = unsafe extern "C" fn(usize, &Arr<f64>, u32, f64, bool, i32, i8, u64, f32, &Arr<usize>, i64, i8) -> bool;


pub type Func13 = unsafe extern "C" fn(i64, &Arr<i8>, u16, f32, &Arr<bool>, &Vec4, &Str, i32, &Vec3, usize, &Vec2, &Arr<u8>, i16) -> Str;


pub type Func14 = unsafe extern "C" fn(&Arr<i8>, &Arr<u32>, &Mat4x4, bool, u16, i32, &Arr<f32>, u16, &Arr<u8>, i8, &Vec3, &Vec4, f64, usize) -> Arr<Str>;


pub type Func15 = unsafe extern "C" fn(&Arr<i16>, &Mat4x4, &Vec4, usize, u64, &Arr<u32>, bool, f32, &Arr<u16>, u8, i32, &Vec2, u16, f64, &Arr<u8>) -> i16;


pub type Func16 = unsafe extern "C" fn(&Arr<bool>, i16, &Arr<i8>, &Vec4, &Mat4x4, &Vec2, &Arr<u64>, &Arr<i8>, &Str, i64, &Arr<u32>, &Vec3, f32, f64, i8, u16) -> usize;


pub type Func17 = unsafe extern "C" fn(&mut i32);


pub type Func18 = unsafe extern "C" fn(&mut i8, &mut i16) -> Vec2;


pub type Func19 = unsafe extern "C" fn(&mut u32, &mut Vec3, &mut Arr<u32>);


pub type Func20 = unsafe extern "C" fn(&mut u16, &mut Vec4, &mut Arr<u64>, &mut i8) -> i32;


pub type Func21 = unsafe extern "C" fn(&mut Mat4x4, &mut Arr<i32>, &mut Vec2, &mut bool, &mut f64) -> f32;


pub type Func22 = unsafe extern "C" fn(&mut usize, &mut u32, &mut Arr<f64>, &mut i16, &mut Str, &mut Vec4) -> u64;


pub type Func23 = unsafe extern "C" fn(&mut u64, &mut Vec2, &mut Arr<i16>, &mut u16, &mut f32, &mut i8, &mut Arr<u8>);


pub type Func24 = unsafe extern "C" fn(&mut Arr<i8>, &mut i64, &mut Arr<u8>, &mut Vec4, &mut u64, &mut Arr<usize>, &mut f64, &mut Arr<usize>) -> Mat4x4;


pub type Func25 = unsafe extern "C" fn(&mut i32, &mut Arr<usize>, &mut bool, &mut u8, &mut Str, &mut Vec3, &mut i64, &mut Vec4, &mut u16) -> f64;


pub type Func26 = unsafe extern "C" fn(&mut u16, &mut Vec2, &mut Mat4x4, &mut Arr<f32>, &mut i16, &mut u64, &mut u32, &mut Arr<u16>, &mut usize, &mut bool) -> i8;


pub type Func27 = unsafe extern "C" fn(&mut f32, &mut Vec3, &mut usize, &mut Vec2, &mut Arr<i16>, &mut Mat4x4, &mut bool, &mut Vec4, &mut i8, &mut i32, &mut Arr<u8>) -> u8;


pub type Func28 = unsafe extern "C" fn(&mut usize, &mut u16, &mut Arr<u32>, &mut Mat4x4, &mut f32, &mut Vec4, &mut Str, &mut Arr<u64>, &mut i64, &mut bool, &mut Vec3, &mut Arr<f32>) -> Str;


pub type Func29 = unsafe extern "C" fn(&mut Vec4, &mut i32, &mut Arr<i8>, &mut f64, &mut bool, &mut i8, &mut Arr<u16>, &mut f32, &mut Str, &mut Mat4x4, &mut u64, &mut Vec3, &mut Arr<i64>) -> Arr<Str>;


pub type Func30 = unsafe extern "C" fn(&mut usize, &mut Vec4, &mut i64, &mut Arr<u32>, &mut bool, &mut Str, &mut Vec3, &mut Arr<u8>, &mut f32, &mut Vec2, &mut Mat4x4, &mut i8, &mut Arr<f32>, &mut f64) -> i32;


pub type Func31 = unsafe extern "C" fn(&mut i8, &mut u32, &mut Arr<u64>, &mut Vec4, &mut Str, &mut bool, &mut i64, &mut Vec2, &mut i8, &mut u16, &mut Arr<i16>, &mut Mat4x4, &mut Vec3, &mut f32, &mut Arr<f64>) -> Vec3;


pub type Func32 = unsafe extern "C" fn(&mut i32, &mut u16, &mut Arr<i8>, &mut Vec4, &mut usize, &mut Arr<u32>, &mut Mat4x4, &mut u64, &mut Str, &mut i64, &mut Vec2, &mut Arr<i8>, &mut bool, &mut Vec3, &mut u8, &mut Arr<u16>) -> f64;


pub type Func33 = unsafe extern "C" fn(&mut Var);


pub type FuncEnum = unsafe extern "C" fn(Example, &mut Arr<Example>) -> Arr<Example>;


