use plugify::{*};
use crate::cross_call_master::*;
type MasterExample = crate::cross_call_master::Example;

// ============================================================================
// Formatting Helper Functions
// ============================================================================

fn format_bool(b: bool) -> &'static str {
    if b { "true" } else { "false" }
}

fn format_vec2(v: &Vec2) -> String {
    format!("{{{}, {}}}", v.x, v.y)
}

fn format_vec3(v: &Vec3) -> String {
    format!("{{{}, {}, {}}}", v.x, v.y, v.z)
}

fn format_vec4(v: &Vec4) -> String {
    format!("{{{}, {}, {}, {}}}", v.x, v.y, v.z, v.w)
}

fn format_mat(m: &Mat4x4) -> String {
    format!(
        "{{{{{}, {}, {}, {}}}, {{{}, {}, {}, {}}}, {{{}, {}, {}, {}}}, {{{}, {}, {}, {}}}}}",
        m[0][0], m[0][1], m[0][2], m[0][3],
        m[1][0], m[1][1], m[1][2], m[1][3],
        m[2][0], m[2][1], m[2][2], m[2][3],
        m[3][0], m[3][1], m[3][2], m[3][3]
    )
}

fn format_any(v: &Var) -> String {
    match v.get() {
        Any::Invalid => String::from("Invalid"),
        Any::Bool(v) => format!("{}", v),
        Any::Char8(v) => format!("{}", v),
        Any::Char16(v) => format!("{}", v),
        Any::Int8(v) => format!("{}", v),
        Any::Int16(v) => format!("{}", v),
        Any::Int32(v) => format!("{}", v),
        Any::Int64(v) => format!("{}", v),
        Any::UInt8(v) => format!("{}", v),
        Any::UInt16(v) => format!("{}", v),
        Any::UInt32(v) => format!("{}", v),
        Any::UInt64(v) => format!("{}", v),
        Any::Pointer(v) => format!("{:p}", v as *const ()),
        Any::Float(v) => format!("{}", v),
        Any::Double(v) => format!("{}", v),
        Any::String(v) => format!("{}", v),
        Any::ArrayBool(v) => VecFormat::format(&v),
        Any::ArrayChar8(v) => VecFormat::format(&v),
        Any::ArrayChar16(v) => VecFormat::format(&v),
        Any::ArrayInt8(v) => VecFormat::format(&v),
        Any::ArrayInt16(v) => VecFormat::format(&v),
        Any::ArrayInt32(v) => VecFormat::format(&v),
        Any::ArrayInt64(v) => VecFormat::format(&v),
        Any::ArrayUInt8(v) => VecFormat::format(&v),
        Any::ArrayUInt16(v) => VecFormat::format(&v),
        Any::ArrayUInt32(v) => VecFormat::format(&v),
        Any::ArrayUInt64(v) => VecFormat::format(&v),
        Any::ArrayPointer(v) => VecFormat::format(&v),
        Any::ArrayFloat(v) => VecFormat::format(&v),
        Any::ArrayDouble(v) => VecFormat::format(&v),
        Any::ArrayString(v) => VecFormat::format(&v),
        Any::ArrayVector2(v) => VecFormat::format(&v),
        Any::ArrayVector3(v) => VecFormat::format(&v),
        Any::ArrayVector4(v) => VecFormat::format(&v),
        Any::ArrayMatrix4x4(v) => VecFormat::format(&v),
        Any::Vector2(v) => format_vec2(&v),
        Any::Vector3(v) => format_vec3(&v),
        Any::Vector4(v) => format_vec4(&v),
    }
}

trait ArrFormat<T> {
    fn format(&self) -> String;
}

impl<T: std::fmt::Display + ArrOps + 'static> ArrFormat<T> for Arr<T> {
    fn format(&self) -> String {
        let elements: Vec<String> = self.iter().map(|x| {
            let any = x as &dyn std::any::Any;
            if let Some(s) = any.downcast_ref::<Str>() {
                format!("'{}'", s)
            } else if let Some(v) = any.downcast_ref::<Var>() {
                format!("{}", format_any(v))
            } else if let Some(v) = any.downcast_ref::<Vec2>() {
                format!("{}", format_vec2(v))
            } else if let Some(v) = any.downcast_ref::<Vec3>() {
                format!("{}", format_vec3(v))
            } else if let Some(v) = any.downcast_ref::<Vec4>() {
                format!("{}", format_vec4(v))
            }  else if let Some(v) = any.downcast_ref::<Mat4x4>() {
                format!("{}", format_mat(v))
            } else if let Some(u) = any.downcast_ref::<usize>() {
                format!("0x{:x}", u)
            } else {
                x.to_string()
            }
        }).collect();
        format!("{{{}}}", elements.join(", "))
    }
}
trait VecFormat<T> {
    fn format(&self) -> String;
}

impl<T: std::fmt::Display> VecFormat<T> for Vec<T> {
    fn format(&self) -> String {
        let elements: Vec<String> = self.iter().map(|x| x.to_string()).collect();
        format!("{{{}}}", elements.join(", "))
    }
}

// ============================================================================
// Example Enum
// ============================================================================

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
    First = 1,
    Second = 2,
    Third = 3,
    Forth = 4,
}
vector_enum_traits!(Example, i32);

use std::fmt;

impl fmt::Display for Example {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Example::First => write!(f, "1"),
            Example::Second => write!(f, "2"),
            Example::Third => write!(f, "3"),
            Example::Forth => write!(f, "4"),
        }
    }
}

// ============================================================================
// NoParam Return Functions
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnVoid() {
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnBool() -> bool {
    true
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnChar8() -> i8 {
    i8::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnChar16() -> u16 {
    u16::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnInt8() -> i8 {
    i8::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnInt16() -> i16 {
    i16::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnInt32() -> i32 {
    i32::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnInt64() -> i64 {
    i64::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnUInt8() -> u8 {
    u8::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnUInt16() -> u16 {
    u16::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnUInt32() -> u32 {
    u32::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnUInt64() -> u64 {
    u64::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnPointer() -> usize {
    0x1usize
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnFloat() -> f32 {
    f32::MAX
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnDouble() -> f64 {
    f64::MAX
}

type NoParamReturnFunctionFunc = extern "C" fn();
extern "C" fn func() {
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnFunction() -> NoParamReturnFunctionFunc {
    func
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnString() -> Str {
    Str::from("Hello World")
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnAny() -> Var {
    let vec = Vec::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    Var::from(Any::from(vec))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayBool() -> Arr<bool> {
    Arr::from(vec![true, false])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayChar8() -> Arr<i8> {
    Arr::from(vec![b'a' as i8, b'b' as i8, b'c' as i8, b'd' as i8])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayChar16() -> Arr<u16> {
    Arr::from(vec!['a' as u16, 'b' as u16, 'c' as u16, 'd' as u16])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayInt8() -> Arr<i8> {
    Arr::from(vec![-3, -2, -1, 0, 1])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayInt16() -> Arr<i16> {
    Arr::from(vec![-4, -3, -2, -1, 0, 1])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayInt32() -> Arr<i32> {
    Arr::from(vec![-5, -4, -3, -2, -1, 0, 1])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayInt64() -> Arr<i64> {
    Arr::from(vec![-6, -5, -4, -3, -2, -1, 0, 1])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayUInt8() -> Arr<u8> {
    Arr::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayUInt16() -> Arr<u16> {
    Arr::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayUInt32() -> Arr<u32> {
    Arr::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayUInt64() -> Arr<u64> {
    Arr::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayPointer() -> Arr<usize> {
    Arr::from(vec![
        0usize,
        1usize,
        2usize,
        3usize,
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayFloat() -> Arr<f32> {
    Arr::from(vec![-12.34f32, 0.0f32, 12.34f32])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayDouble() -> Arr<f64> {
    Arr::from(vec![-12.345, 0.0, 12.345])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayString() -> Arr<Str> {
    Arr::from(vec![
        Str::from("1st string"),
        Str::from("2nd string"),
        Str::from("3rd element string (Should be big enough to avoid small string optimization)"),
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayAny() -> Arr<Var> {
    Arr::from(vec![
        Any::from(1.0_f64),
        Any::from(2.0_f32),
        Any::from("3rd element string (Should be big enough to avoid small string optimization)"),
        Any::from(Vec::from(vec![
            String::from("lolek"),
            String::from("and"),
            String::from("bolek"),
        ])),
        Any::from(1_i32),
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayVector2() -> Arr<Vec2> {
    Arr::from(vec![
        Vec2 { x: 1.1, y: 2.2 },
        Vec2 { x: -3.3, y: 4.4 },
        Vec2 { x: 5.5, y: -6.6 },
        Vec2 { x: 7.7, y: 8.8 },
        Vec2 { x: 0.0, y: 0.0 },
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayVector3() -> Arr<Vec3> {
    Arr::from(vec![
        Vec3 { x: 1.1, y: 2.2, z: 3.3 },
        Vec3 { x: -4.4, y: 5.5, z: -6.6 },
        Vec3 { x: 7.7, y: 8.8, z: 9.9 },
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        Vec3 { x: 10.1, y: -11.2, z: 12.3 },
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayVector4() -> Arr<Vec4> {
    Arr::from(vec![
        Vec4 { x: 1.1, y: 2.2, z: 3.3, w: 4.4 },
        Vec4 { x: -5.5, y: 6.6, z: -7.7, w: 8.8 },
        Vec4 { x: 9.9, y: 0.0, z: -1.1, w: 2.2 },
        Vec4 { x: 3.3, y: 4.4, z: 5.5, w: 6.6 },
        Vec4 { x: -7.7, y: -8.8, z: 9.9, w: -10.1 },
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayMatrix4x4() -> Arr<Mat4x4> {
    Arr::from(vec![
        Mat4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0], // Identity matrix
            ]
        },
        Mat4x4 {
            m: [
                [2.0, 3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0, 9.0],
                [10.0, 11.0, 12.0, 13.0],
                [14.0, 15.0, 16.0, 17.0], // Example random matrix
            ]
        },
        Mat4x4 {
            m: [
                [-1.0, -2.0, -3.0, -4.0],
                [-5.0, -6.0, -7.0, -8.0],
                [-9.0, -10.0, -11.0, -12.0],
                [-13.0, -14.0, -15.0, -16.0], // Negative matrix
            ]
        },
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnVector2() -> Vec2 {
    Vec2 { x: 1.0, y: 2.0 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnVector3() -> Vec3 {
    Vec3 { x: 1.0, y: 2.0, z: 3.0 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnVector4() -> Vec4 {
    Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnMatrix4x4() -> Mat4x4 {
    Mat4x4 {
        m: [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 14.0, 15.0, 16.0]]
    }
}

// ============================================================================
// Param Functions
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param1(a: i32) {
    let _buffer = format!("{}", a);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param2(a: i32, b: f32) {
    let _buffer = format!("{}{:.2}", a, b);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param3(a: i32, b: f32, c: f64) {
    let _buffer = format!("{}{:.2}{:.2}", a, b, c);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param4(a: i32, b: f32, c: f64, d: &Vec4) {
    let _buffer = format!("{}{:.2}{:.2}{:.1}{:.1}", a, b, c, d.x, d.w);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param5(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>) {
    let _buffer = format!(
        "{}{:.2}{:.2}{:.1}{:.1}{}{}",
        a,
        b,
        c,
        d.x,
        d.w,
        e.len(),
        if e.len() == 3 { e.get(2).copied().unwrap_or(0) } else { 0 }
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param6(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8) {
    let _buffer = format!(
        "{}{:.2}{:.2}{:.1}{:.1}{}{}{}",
        a,
        b,
        c,
        d.x,
        d.w,
        e.len(),
        if e.len() == 3 { e.get(2).copied().unwrap_or(0) } else { 0 },
        f as u8
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param7(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8, g: &Str) {
    let _buffer = format!(
        "{}{:.2}{:.2}{:.1}{:.1}{}{}{}{}",
        a,
        b,
        c,
        d.x,
        d.w,
        e.len(),
        if e.len() == 3 { e.get(2).copied().unwrap_or(0) } else { 0 },
        f as u8,
        g
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param8(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8, g: &Str, h: u16) {
    let _buffer = format!(
        "{}{:.2}{:.2}{:.1}{:.1}{}{}{}{}{}",
        a,
        b,
        c,
        d.x,
        d.w,
        e.len(),
        if e.len() == 3 { e.get(2).copied().unwrap_or(0) } else { 0 },
        f as u8,
        g,
        h
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param9(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8, g: &Str, h: u16, k: i16) {
    let _buffer = format!(
        "{}{:.2}{:.2}{:.1}{:.1}{}{}{}{}{}{}",
        a,
        b,
        c,
        d.x,
        d.w,
        e.len(),
        if e.len() == 3 { e.get(2).copied().unwrap_or(0) } else { 0 },
        f as u8,
        g,
        h,
        k
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param10(a: i32, b: f32, c: f64, d: &Vec4, e: &Arr<i64>, f: i8, g: &Str, h: u16, k: i16, l: usize) {
    let _buffer = format!(
        "{}{:.2}{:.2}{:.1}{:.1}{}{}{}{}{}{}{}",
        a,
        b,
        c,
        d.x,
        d.w,
        e.len(),
        if e.len() == 3 { e.get(2).copied().unwrap_or(0) } else { 0 },
        f as u8,
        g,
        h,
        k,
        l
    );
}

// ============================================================================
// ParamRef Functions
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef1(a: &mut i32) {
    *a = 42;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef2(a: &mut i32, b: &mut f32) {
    *a = 10;
    *b = 3.14;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef3(a: &mut i32, b: &mut f32, c: &mut f64) {
    *a = -20;
    *b = 2.718;
    *c = 3.14159;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef4(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4) {
    *a = 100;
    *b = -5.55;
    *c = 1.618;
    *d = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef5(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>) {
    *a = 500;
    *b = -10.5;
    *c = 2.71828;
    *d = Vec4 { x: -1.0, y: -2.0, z: -3.0, w: -4.0 };
    e.set(&[-6, -5, -4, -3, -2, -1, 0, 1]);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef6(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8) {
    *a = 750;
    *b = 20.0;
    *c = 1.23456;
    *d = Vec4 { x: 10.0, y: 20.0, z: 30.0, w: 40.0 };
    e.set(&[-6, -5, -4]);
    *f = b'Z' as i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef7(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8, g: &mut Str) {
    *a = -1000;
    *b = 3.0;
    *c = -1.0;
    *d = Vec4 { x: 100.0, y: 200.0, z: 300.0, w: 400.0 };
    e.set(&[-6, -5, -4, -3]);
    *f = b'Y' as i8;
    g.set("Hello, World!");
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef8(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8, g: &mut Str, h: &mut u16) {
    *a = 999;
    *b = -7.5;
    *c = 0.123456;
    *d = Vec4 { x: -100.0, y: -200.0, z: -300.0, w: -400.0 };
    e.set(&[-6, -5, -4, -3, -2, -1]);
    *f = b'X' as i8;
    g.set("Goodbye, World!");
    *h = 'A' as u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef9(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8, g: &mut Str, h: &mut u16, k: &mut i16) {
    *a = -1234;
    *b = 123.45;
    *c = -678.9;
    *d = Vec4 { x: 987.65, y: 432.1, z: 123.456, w: 789.123 };
    e.set(&[-6, -5, -4, -3, -2, -1, 0, 1, 5, 9]);
    *f = b'W' as i8;
    g.set("Testing, 1 2 3");
    *h = 'B' as u16;
    *k = 42;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef10(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vec4, e: &mut Arr<i64>, f: &mut i8, g: &mut Str, h: &mut u16, k: &mut i16, l: &mut usize) {
    *a = 987;
    *b = -0.123;
    *c = 456.789;
    *d = Vec4 { x: -123.456, y: 0.987, z: 654.321, w: -789.123 };
    e.set(&[-6, -5, -4, -3, -2, -1, 0, 1, 5, 9, 4, -7]);
    *f = b'V' as i8;
    g.set("Another string");
    *h = 'C' as u16;
    *k = -444;
    *l = 0x12345678usize;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRefVectors(
    p1: &mut Arr<bool>,
    p2: &mut Arr<i8>,
    p3: &mut Arr<u16>,
    p4: &mut Arr<i8>,
    p5: &mut Arr<i16>,
    p6: &mut Arr<i32>,
    p7: &mut Arr<i64>,
    p8: &mut Arr<u8>,
    p9: &mut Arr<u16>,
    p10: &mut Arr<u32>,
    p11: &mut Arr<u64>,
    p12: &mut Arr<usize>,
    p13: &mut Arr<f32>,
    p14: &mut Arr<f64>,
    p15: &mut Arr<Str>,
) {
    p1.set(&[true]);
    p2.set(&[b'a' as i8, b'b' as i8, b'c' as i8]);
    p3.set(&['d' as u16, 'e' as u16, 'f' as u16]);
    p4.set(&[-3, -2, -1, 0, 1, 2, 3]);
    p5.set(&[-4, -3, -2, -1, 0, 1, 2, 3, 4]);
    p6.set(&[-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]);
    p7.set(&[-6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6]);
    p8.set(&[0, 1, 2, 3, 4, 5, 6, 7]);
    p9.set(&[0, 1, 2, 3, 4, 5, 6, 7, 8]);
    p10.set(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    p11.set(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    p12.set(&[0usize, 1usize, 2usize]);
    p13.set(&[-12.34, 0.0, 12.34]);
    p14.set(&[-12.345, 0.0, 12.345]);

    let strings = vec![
        Str::from("1"),
        Str::from("12"),
        Str::from("123"),
        Str::from("1234"),
        Str::from("12345"),
        Str::from("123456"),
    ];
    p15.set(&strings);
}

// ============================================================================
// ParamAllPrimitives and Variant Functions
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamAllPrimitives(
    p1: bool,
    p2: i8,
    p3: u16,
    p4: i8,
    p5: i16,
    p6: i32,
    p7: i64,
    p8: u8,
    p9: u16,
    p10: u32,
    p11: u64,
    p12: usize,
    p13: f32,
    p14: f64,
) -> i64 {
    let _buffer = format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        p1 as u8,
        p2 as u8,
        p3 as u8,
        p4,
        p5,
        p6,
        p7,
        p8,
        p9,
        p10,
        p11,
        p12,
        p13,
        p14
    );
    56
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamEnum(p1: Example, p2: &Arr<Example>) -> i32 {
    let sum: i32 = p2.iter().map(|e| *e as i32).sum();
    (p1 as i32) + sum
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamEnumRef(p1: &mut Example, p2: &mut Arr<Example>) -> i32 {
    *p1 = Example::Forth;

    let new_values = vec![Example::First, Example::Second, Example::Third];
    p2.set(&new_values);

    let sum: i32 = p2.iter().map(|e| *e as i32).sum();
    (*p1 as i32) + sum
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamVariant(p1: &Var, p2: &Arr<Var>) {
    let _buffer = format!("{:?}|{}", p1.get(), p2.len());
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamVariantRef(p1: &mut Var, p2: &mut Arr<Var>) {
    p1.set(&Any::Char8(b'Z' as i8));

    let variants = vec![
        Var::new(&Any::Bool(false)),
        Var::new(&Any::Double(6.28)),
        Var::new(&Any::ArrayDouble(vec![1.0, 2.0, 3.0])),
        Var::new(&Any::Pointer(0)),
        Var::new(&Any::Int32(123456789)),
    ];
    p2.set(&variants);
}

// ============================================================================
// Function Pointer Type Aliases
// ============================================================================

type FuncVoid = extern "C" fn();
type FuncBool = extern "C" fn() -> bool;
type FuncChar8 = extern "C" fn() -> i8;
type FuncChar16 = extern "C" fn() -> u16;
type FuncInt8 = extern "C" fn() -> i8;
type FuncInt16 = extern "C" fn() -> i16;
type FuncInt32 = extern "C" fn() -> i32;
type FuncInt64 = extern "C" fn() -> i64;
type FuncUInt8 = extern "C" fn() -> u8;
type FuncUInt16 = extern "C" fn() -> u16;
type FuncUInt32 = extern "C" fn() -> u32;
type FuncUInt64 = extern "C" fn() -> u64;
type FuncFloat = extern "C" fn() -> f32;
type FuncDouble = extern "C" fn() -> f64;
type FuncString = extern "C" fn() -> Str;
type FuncAny = extern "C" fn() -> Var;

type FuncBoolVector = extern "C" fn() -> Arr<bool>;
type FuncChar8Vector = extern "C" fn() -> Arr<i8>;
type FuncChar16Vector = extern "C" fn() -> Arr<u16>;
type FuncInt8Vector = extern "C" fn() -> Arr<i8>;
type FuncInt16Vector = extern "C" fn() -> Arr<i16>;
type FuncInt32Vector = extern "C" fn() -> Arr<i32>;
type FuncInt64Vector = extern "C" fn() -> Arr<i64>;
type FuncUInt8Vector = extern "C" fn() -> Arr<u8>;
type FuncUInt16Vector = extern "C" fn() -> Arr<u16>;
type FuncUInt32Vector = extern "C" fn() -> Arr<u32>;
type FuncUInt64Vector = extern "C" fn() -> Arr<u64>;
type FuncPtrVector = extern "C" fn() -> Arr<usize>;
type FuncFloatVector = extern "C" fn() -> Arr<f32>;
type FuncDoubleVector = extern "C" fn() -> Arr<f64>;
type FuncStringVector = extern "C" fn() -> Arr<Str>;
type FuncAnyVector = extern "C" fn() -> Arr<Var>;
type FuncVec2Vector = extern "C" fn() -> Arr<Vec2>;
type FuncVec3Vector = extern "C" fn() -> Arr<Vec3>;
type FuncVec4Vector = extern "C" fn() -> Arr<Vec4>;
type FuncMat4x4Vector = extern "C" fn() -> Arr<Mat4x4>;

type FuncVec2 = extern "C" fn() -> Vec2;
type FuncVec3 = extern "C" fn() -> Vec3;
type FuncVec4 = extern "C" fn() -> Vec4;
type FuncMat4x4 = extern "C" fn() -> Mat4x4;

type Func1 = extern "C" fn(&Vec3) -> i32;
type Func2 = extern "C" fn(f32, i64) -> i8;
type Func3 = extern "C" fn(usize, &Vec4, &Str);
type Func4 = extern "C" fn(bool, i32, u16, &Mat4x4) -> Vec4;
type Func5 = extern "C" fn(i8, &Vec2, usize, f64, &Arr<u64>) -> bool;
type Func6 = extern "C" fn(&Str, f32, &Arr<f32>, i16, &Arr<u8>, usize) -> i64;
type Func7 = extern "C" fn(&Arr<i8>, u16, u16, &Arr<u32>, &Vec4, bool, u64) -> f64;
type Func8 = extern "C" fn(&Vec3, &Arr<u32>, i16, bool, &Vec4, &Arr<u16>, u16, i32) -> Mat4x4;
type Func9 = extern "C" fn(f32, &Vec2, &Arr<i8>, u64, bool, &Str, &Vec4, i16, usize);
type Func10 = extern "C" fn(&Vec4, &Mat4x4, &Arr<u32>, u64, &Arr<i8>, i32, bool, &Vec2, i64, f64) -> u32;
type Func11 = unsafe extern "C" fn(&Arr<bool>, u16, u8, f64, &Vec3, &Arr<i8>, i64, u16, f32, &Vec2, u32) -> usize;
type Func12 = extern "C" fn(usize, &Arr<f64>, u32, f64, bool, i32, i8, u64, f32, &Arr<usize>, i64, i8) -> bool;
type Func13 = extern "C" fn(i64, &Arr<i8>, u16, f32, &Arr<bool>, &Vec4, &Str, i32, &Vec3, usize, &Vec2, &Arr<u8>, i16) -> Str;
type Func14 = extern "C" fn(&Arr<i8>, &Arr<u32>, &Mat4x4, bool, u16, i32, &Arr<f32>, u16, &Arr<u8>, i8, &Vec3, &Vec4, f64, usize) -> Arr<Str>;
type Func15 = extern "C" fn(&Arr<i16>, &Mat4x4, &Vec4, usize, u64, &Arr<u32>, bool, f32, &Arr<u16>, u8, i32, &Vec2, u16, f64, &Arr<u8>) -> i16;
type Func16 = unsafe extern "C" fn(&Arr<bool>, i16, &Arr<i8>, &Vec4, &Mat4x4, &Vec2, &Arr<u64>, &Arr<i8>, &Str, i64, &Arr<u32>, &Vec3, f32, f64, i8, u16) -> usize;
type Func17 = extern "C" fn(&mut i32);
type Func18 = extern "C" fn(&mut i8, &mut i16) -> Vec2;
type Func19 = extern "C" fn(&mut u32, &mut Vec3, &mut Arr<u32>);
type Func20 = extern "C" fn(&mut u16, &mut Vec4, &mut Arr<u64>, &mut i8) -> i32;
type Func21 = extern "C" fn(&mut Mat4x4, &mut Arr<i32>, &mut Vec2, &mut bool, &mut f64) -> f32;
type Func22 = extern "C" fn(&mut usize, &mut u32, &mut Arr<f64>, &mut i16, &mut Str, &mut Vec4) -> u64;
type Func23 = extern "C" fn(&mut u64, &mut Vec2, &mut Arr<i16>, &mut u16, &mut f32, &mut i8, &mut Arr<u8>);
type Func24 = extern "C" fn(&mut Arr<i8>, &mut i64, &mut Arr<u8>, &mut Vec4, &mut u64, &mut Arr<usize>, &mut f64, &mut Arr<usize>) -> Mat4x4;
type Func25 = extern "C" fn(&mut i32, &mut Arr<usize>, &mut bool, &mut u8, &mut Str, &mut Vec3, &mut i64, &mut Vec4, &mut u16) -> f64;
type Func26 = extern "C" fn(&mut u16, &mut Vec2, &mut Mat4x4, &mut Arr<f32>, &mut i16, &mut u64, &mut u32, &mut Arr<u16>, &mut usize, &mut bool) -> i8;
type Func27 = extern "C" fn(&mut f32, &mut Vec3, &mut usize, &mut Vec2, &mut Arr<i16>, &mut Mat4x4, &mut bool, &mut Vec4, &mut i8, &mut i32, &mut Arr<u8>) -> u8;
type Func28 = extern "C" fn(&mut usize, &mut u16, &mut Arr<u32>, &mut Mat4x4, &mut f32, &mut Vec4, &mut Str, &mut Arr<u64>, &mut i64, &mut bool, &mut Vec3, &mut Arr<f32>) -> Str;
type Func29 = extern "C" fn(&mut Vec4, &mut i32, &mut Arr<i8>, &mut f64, &mut bool, &mut i8, &mut Arr<u16>, &mut f32, &mut Str, &mut Mat4x4, &mut u64, &mut Vec3, &mut Arr<i64>) -> Arr<Str>;
type Func30 = extern "C" fn(&mut usize, &mut Vec4, &mut i64, &mut Arr<u32>, &mut bool, &mut Str, &mut Vec3, &mut Arr<u8>, &mut f32, &mut Vec2, &mut Mat4x4, &mut i8, &mut Arr<f32>, &mut f64) -> i32;
type Func31 = extern "C" fn(&mut i8, &mut u32, &mut Arr<u64>, &mut Vec4, &mut Str, &mut bool, &mut i64, &mut Vec2, &mut i8, &mut u16, &mut Arr<i16>, &mut Mat4x4, &mut Vec3, &mut f32, &mut Arr<f64>) -> Vec3;
type Func32 = extern "C" fn(&mut i32, &mut u16, &mut Arr<i8>, &mut Vec4, &mut usize, &mut Arr<u32>, &mut Mat4x4, &mut u64, &mut Str, &mut i64, &mut Vec2, &mut Arr<i8>, &mut bool, &mut Vec3, &mut u8, &mut Arr<u16>) -> f64;
type Func33 = extern "C" fn(&mut Var);
type FuncEnum = extern "C" fn(Example, &mut Arr<Example>) -> Arr<Example>;

// ============================================================================
// CallFunc Functions - Simple
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVoid(func: FuncVoid) {
    func();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncBool(func: FuncBool) -> bool {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncChar8(func: FuncChar8) -> i8 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncChar16(func: FuncChar16) -> u16 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt8(func: FuncInt8) -> i8 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt16(func: FuncInt16) -> i16 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt32(func: FuncInt32) -> i32 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt64(func: FuncInt64) -> i64 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt8(func: FuncUInt8) -> u8 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt16(func: FuncUInt16) -> u16 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt32(func: FuncUInt32) -> u32 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt64(func: FuncUInt64) -> u64 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncFloat(func: FuncFloat) -> f32 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncDouble(func: FuncDouble) -> f64 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncPtr(func: FuncPtr) -> usize {
    unsafe { func() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncString(func: FuncString) -> Str {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncAny(func: FuncAny) -> Var {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncFunction(func: FuncFunction) -> usize {
    unsafe { func() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncBoolVector(func: FuncBoolVector) -> Arr<bool> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncChar8Vector(func: FuncChar8Vector) -> Arr<i8> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncChar16Vector(func: FuncChar16Vector) -> Arr<u16> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt8Vector(func: FuncInt8Vector) -> Arr<i8> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt16Vector(func: FuncInt16Vector) -> Arr<i16> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt32Vector(func: FuncInt32Vector) -> Arr<i32> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt64Vector(func: FuncInt64Vector) -> Arr<i64> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt8Vector(func: FuncUInt8Vector) -> Arr<u8> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt16Vector(func: FuncUInt16Vector) -> Arr<u16> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt32Vector(func: FuncUInt32Vector) -> Arr<u32> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt64Vector(func: FuncUInt64Vector) -> Arr<u64> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncPtrVector(func: FuncPtrVector) -> Arr<usize> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncFloatVector(func: FuncFloatVector) -> Arr<f32> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncDoubleVector(func: FuncDoubleVector) -> Arr<f64> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncStringVector(func: FuncStringVector) -> Arr<Str> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncAnyVector(func: FuncAnyVector) -> Arr<Var> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec2Vector(func: FuncVec2Vector) -> Arr<Vec2> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec3Vector(func: FuncVec3Vector) -> Arr<Vec3> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec4Vector(func: FuncVec4Vector) -> Arr<Vec4> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncMat4x4Vector(func: FuncMat4x4Vector) -> Arr<Mat4x4> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec2(func: FuncVec2) -> Vec2 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec3(func: FuncVec3) -> Vec3 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec4(func: FuncVec4) -> Vec4 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncMat4x4(func: FuncMat4x4) -> Mat4x4 {
    func()
}

// ============================================================================
// CallFunc Functions - Complex (with parameters)
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc1(func: Func1) -> i32 {
    let vec = Vec3 { x: 4.5, y: 5.6, z: 6.7 };
    func(&vec)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc2(func: Func2) -> i8 {
    let f = 2.71f32;
    let i64 = 200i64;
    func(f, i64)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc3(func: Func3) {
    let ptr = 12345usize;
    let vec4 = Vec4 { x: 7.8, y: 8.9, z: 9.1, w: 10.2 };
    let str = Str::from("RandomString");
    func(ptr, &vec4, &str);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc4(func: Func4) -> Vec4 {
    let b = false;
    let u32 = 42i32;
    let ch16 = 'B' as u16;
    let mat = Mat4x4 { m: [[0.0; 4]; 4] };
    func(b, u32, ch16, &mat)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc5(func: Func5) -> bool {
    let i8 = 10i8;
    let vec2 = Vec2 { x: 3.4, y: 5.6 };
    let ptr = 67890usize;
    let d = 1.618f64;
    let vec64 = Arr::from(vec![4u64, 5, 6]);
    func(i8, &vec2, ptr, d, &vec64)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc6(func: Func6) -> i64 {
    let str = Str::from("AnotherString");
    let f = 4.56f32;
    let vec_f = Arr::from(vec![4.0f32, 5.0, 6.0]);
    let i16 = 30i16;
    let vec_u8 = Arr::from(vec![3u8, 4, 5]);
    let ptr = 24680usize;
    func(&str, f, &vec_f, i16, &vec_u8, ptr)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc7(func: Func7) -> f64 {
    let vec_c = Arr::from(vec![b'X' as i8, b'Y' as i8, b'Z' as i8]);
    let u16 = 20u16;
    let ch16 = 'C' as u16;
    let vec_u32 = Arr::from(vec![4u32, 5, 6]);
    let vec4 = Vec4 { x: 4.5, y: 5.6, z: 6.7, w: 7.8 };
    let b = false;
    let u64 = 200u64;
    func(&vec_c, u16, ch16, &vec_u32, &vec4, b, u64)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc8(func: Func8) -> Mat4x4 {
    let vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let vec_u32 = Arr::from(vec![4u32, 5, 6]);
    let i16 = 30i16;
    let b = false;
    let vec4 = Vec4 { x: 4.5, y: 5.6, z: 6.7, w: 7.8 };
    let vec_c16 = Arr::from(vec!['D' as u16, 'E' as u16]);
    let ch16 = 'B' as u16;
    let i32 = 50i32;
    func(&vec3, &vec_u32, i16, b, &vec4, &vec_c16, ch16, i32)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc9(func: Func9) {
    let f = 2.71f32;
    let vec2 = Vec2 { x: 3.4, y: 5.6 };
    let vec_i8 = Arr::from(vec![4i8, 5, 6]);
    let u64 = 250u64;
    let b = false;
    let str = Str::from("Random");
    let vec4 = Vec4 { x: 4.5, y: 5.6, z: 6.7, w: 7.8 };
    let i16 = 30i16;
    let ptr = 13579usize;
    func(f, &vec2, &vec_i8, u64, b, &str, &vec4, i16, ptr);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc10(func: Func10) -> u32 {
    let vec4 = Vec4 { x: 5.6, y: 7.8, z: 8.9, w: 9.0 };
    let mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let vec_u32 = Arr::from(vec![4u32, 5, 6]);
    let u64 = 150u64;
    let vec_c = Arr::from(vec![b'X' as i8, b'Y' as i8, b'Z' as i8]);
    let i32 = 60i32;
    let b = false;
    let vec2 = Vec2 { x: 3.4, y: 5.6 };
    let i64 = 75i64;
    let d = 2.71f64;
    func(&vec4, &mat, &vec_u32, u64, &vec_c, i32, b, &vec2, i64, d)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc11(func: Func11) -> usize {
    let vec_b = Arr::from(vec![false, true, false]);
    let ch16 = 'C' as u16;
    let u8_val: u8 = 10;
    let d: f64 = 2.71;
    let vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let vec_i8 = Arr::from(vec![3, 4, 5]);
    let i64_val: i64 = 150;
    let u16_val: u16 = 20;
    let f: f32 = 2.0;
    let vec2 = Vec2 { x: 4.5, y: 6.7 };
    let u32_val: u32 = 30;

    unsafe { func(&vec_b, ch16, u8_val, d, &vec3, &vec_i8, i64_val, u16_val, f, &vec2, u32_val) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc12(func: Func12) -> bool {
    let ptr = 98765usize;
    let vec_d = Arr::from(vec![4.0f64, 5.0, 6.0]);
    let u32 = 30u32;
    let d = 1.41f64;
    let b = false;
    let i32 = 25i32;
    let i8 = 10i8;
    let u64 = 300u64;
    let f = 2.72f32;
    let vec_ptr = Arr::from(vec![2usize, 3usize, 4usize]);
    let i64 = 200i64;
    let ch = b'B' as i8;
    func(ptr, &vec_d, u32, d, b, i32, i8, u64, f, &vec_ptr, i64, ch)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc13(func: Func13) -> Str {
    let i64 = 75i64;
    let vec_c = Arr::from(vec![b'D' as i8, b'E' as i8, b'F' as i8]);
    let u16 = 20u16;
    let f = 2.71f32;
    let vec_b = Arr::from(vec![false, true, false]);
    let vec4 = Vec4 { x: 5.6, y: 7.8, z: 9.0, w: 10.1 };
    let str = Str::from("RandomString");
    let i32 = 30i32;
    let vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let ptr = 13579usize;
    let vec2 = Vec2 { x: 4.5, y: 6.7 };
    let vec_u8 = Arr::from(vec![2u8, 3, 4]);
    let i16 = 20i16;
    func(i64, &vec_c, u16, f, &vec_b, &vec4, &str, i32, &vec3, ptr, &vec2, &vec_u8, i16)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc14(func: Func14) -> Arr<Str> {
    let vec_c = Arr::from(vec![b'D' as i8, b'E' as i8, b'F' as i8]);
    let vec_u32 = Arr::from(vec![4u32, 5, 6]);
    let mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let b = false;
    let ch16 = 'B' as u16;
    let i32 = 25i32;
    let vec_f = Arr::from(vec![4.0f32, 5.0, 6.0]);
    let u16 = 30u16;
    let vec_u8 = Arr::from(vec![3u8, 4, 5]);
    let i8 = 10i8;
    let vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let vec4 = Vec4 { x: 5.6, y: 7.8, z: 9.0, w: 10.1 };
    let d = 2.72f64;
    let ptr = 54321usize;
    func(&vec_c, &vec_u32, &mat, b, ch16, i32, &vec_f, u16, &vec_u8, i8, &vec3, &vec4, d, ptr)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc15(func: Func15) -> i16 {
    let vec_i16 = Arr::from(vec![4i16, 5, 6]);
    let mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let vec4 = Vec4 { x: 7.8, y: 8.9, z: 9.0, w: 10.1 };
    let ptr = 12345usize;
    let u64 = 200u64;
    let vec_u32 = Arr::from(vec![5u32, 6, 7]);
    let b = false;
    let f = 3.14f32;
    let vec_c16 = Arr::from(vec!['D' as u16, 'E' as u16]);
    let u8 = 6u8;
    let i32 = 25i32;
    let vec2 = Vec2 { x: 5.6, y: 7.8 };
    let u16 = 40u16;
    let d = 2.71f64;
    let vec_u8 = Arr::from(vec![1u8, 3, 5]);
    func(&vec_i16, &mat, &vec4, ptr, u64, &vec_u32, b, f, &vec_c16, u8, i32, &vec2, u16, d, &vec_u8)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc16(func: Func16) -> usize {
    let vec_b = Arr::from(vec![true, true, false]);
    let i16_val: i16 = 20;
    let vec_i8 = Arr::from(vec![2, 3, 4]);
    let vec4 = Vec4 { x: 7.8, y: 8.9, z: 9.0, w: 10.1 };
    let mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let vec2 = Vec2 { x: 5.6, y: 7.8 };
    let vec_u64 = Arr::from(vec![5, 6, 7]);
    let vec_c = Arr::from(vec!['D' as i8, 'E' as i8, 'F' as i8]);
    let str_val = Str::from("DifferentString");
    let i64_val: i64 = 300;
    let vec_u32 = Arr::from(vec![6, 7, 8]);
    let vec3 = Vec3 { x: 5.0, y: 6.0, z: 7.0 };
    let f: f32 = 3.14;
    let d: f64 = 2.718;
    let i8_val: i8 = 6;
    let u16_val: u16 = 30;

    unsafe {
        func(
            &vec_b, i16_val, &vec_i8, &vec4, &mat, &vec2, &vec_u64, &vec_c,
            &str_val, i64_val, &vec_u32, &vec3, f, d, i8_val, u16_val
        )
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc17(func: Func17) -> Str {
    let mut i32 = 42i32;
    func(&mut i32);
    Str::from(format!("{}", i32))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc18(func: Func18) -> Str {
    let mut i8 = 9i8;
    let mut i16 = 25i16;
    let ret = func(&mut i8, &mut i16);
    Str::from(format!("{}|{}|{}", format_vec2(&ret), i8, i16))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc19(func: Func19) -> Str {
    let mut u32 = 75u32;
    let mut vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut vec_u32 = Arr::from(vec![4u32, 5, 6]);
    func(&mut u32, &mut vec3, &mut vec_u32);
    Str::from(format!("{}|{}|{}", u32, format_vec3(&vec3), ArrFormat::format(&vec_u32)))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc20(func: Func20) -> Str {
    let mut ch16 = 'Z' as u16;
    let mut vec4 = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut vec_u64 = Arr::from(vec![4u64, 5, 6]);
    let mut ch = b'X' as i8;
    let ret = func(&mut ch16, &mut vec4, &mut vec_u64, &mut ch);
    Str::from(format!("{}|{}|{}|{}|{}", ret, ch16, format_vec4(&vec4), ArrFormat::format(&vec_u64), ch as u8 as char))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc21(func: Func21) -> Str {
    let mut mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let mut vec_i32 = Arr::from(vec![4i32, 5, 6]);
    let mut vec2 = Vec2 { x: 3.0, y: 4.0 };
    let mut b = false;
    let mut d = 6.28f64;
    let ret = func(&mut mat, &mut vec_i32, &mut vec2, &mut b, &mut d);
    Str::from(format!(
        "{}|{}|{}|{}|{}|{}",
        ret,
        format_mat(&mat),
        ArrFormat::format(&vec_i32),
        format_vec2(&vec2),
        format_bool(b),
        d
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc22(func: Func22) -> Str {
    let mut ptr = 1usize;
    let mut u32 = 20u32;
    let mut vec_d = Arr::from(vec![4.0f64, 5.0, 6.0]);
    let mut i16 = 15i16;
    let mut str = Str::from("Updated Test");
    let mut vec4 = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let ret = func(&mut ptr, &mut u32, &mut vec_d, &mut i16, &mut str, &mut vec4);
    Str::from(format!(
        "{}|{:p}|{}|{}|{}|{}|{}",
        ret,
        ptr as *const(),
        u32,
        ArrFormat::format(&vec_d),
        i16,
        str.to_string(),
        format_vec4(&vec4)
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc23(func: Func23) -> Str {
    let mut u64 = 200u64;
    let mut vec2 = Vec2 { x: 3.0, y: 4.0 };
    let mut vec_i16 = Arr::from(vec![4i16, 5, 6]);
    let mut ch16 = 'Y' as u16;
    let mut f = 2.34f32;
    let mut i8 = 10i8;
    let mut vec_u8 = Arr::from(vec![3u8, 4, 5]);
    func(&mut u64, &mut vec2, &mut vec_i16, &mut ch16, &mut f, &mut i8, &mut vec_u8);
    Str::from(format!(
        "{}|{}|{}|{}|{}|{}|{}",
        u64,
        format_vec2(&vec2),
        ArrFormat::format(&vec_i16),
        (ch16),
        f,
        i8,
        ArrFormat::format(&vec_u8)
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc24(func: Func24) -> Str {
    let mut vec_c = Arr::from(vec![b'D' as i8, b'E' as i8, b'F' as i8]);
    let mut i64 = 100i64;
    let mut vec_u8 = Arr::from(vec![3u8, 4, 5]);
    let mut vec4 = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut u64 = 200u64;
    let mut vec_ptr = Arr::from(vec![3usize, 4usize, 5usize]);
    let mut d = 6.28f64;
    let mut vec_v2 = Arr::from(vec![4usize, 5usize, 6usize, 7usize]);
    let ret = func(&mut vec_c, &mut i64, &mut vec_u8, &mut vec4, &mut u64, &mut vec_ptr, &mut d, &mut vec_v2);
    Str::from(format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}",
        format_mat(&ret),
        ArrFormat::format(&vec_c),
        i64,
        ArrFormat::format(&vec_u8),
        format_vec4(&vec4),
        u64,
        ArrFormat::format(&vec_ptr),
        d,
        ArrFormat::format(&vec_v2)
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc25(func: Func25) -> Str {
    let mut i32 = 50i32;
    let mut vec_ptr = Arr::from(vec![3usize, 4usize, 5usize]);
    let mut b = false;
    let mut u8 = 10u8;
    let mut str = Str::from("Updated Test String");
    let mut vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut i64 = 100i64;
    let mut vec4 = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut u16 = 20u16;
    let ret = func(&mut i32, &mut vec_ptr, &mut b, &mut u8, &mut str, &mut vec3, &mut i64, &mut vec4, &mut u16);
    Str::from(format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        ret,
        i32,
        ArrFormat::format(&vec_ptr),
        format_bool(b),
        u8,
        str.to_string(),
        format_vec3(&vec3),
        i64,
        format_vec4(&vec4),
        u16
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc26(func: Func26) -> Str {
    let mut ch16 = 'B' as u16;
    let mut vec2 = Vec2 { x: 3.0, y: 4.0 };
    let mut mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let mut vec_f = Arr::from(vec![4.0f32, 5.0, 6.0]);
    let mut i16 = 20i16;
    let mut u64 = 200u64;
    let mut u32 = 20u32;
    let mut vec_u16 = Arr::from(vec![3u16, 4, 5]);
    let mut ptr = 0xDEADBEAFDEADBEAFusize;
    let mut b = false;
    let ret = func(&mut ch16, &mut vec2, &mut mat, &mut vec_f, &mut i16, &mut u64, &mut u32, &mut vec_u16, &mut ptr, &mut b);
    Str::from(format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{:p}|{}",
        ret as u8 as char,
        ch16,
        format_vec2(&vec2),
        format_mat(&mat),
        ArrFormat::format(&vec_f),
        u64,
        u32,
        ArrFormat::format(&vec_u16),
        ptr as *const(),
        format_bool(b)
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc27(func: Func27) -> Str {
    let mut f = 2.56f32;
    let mut vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut ptr = 0usize;
    let mut vec2 = Vec2 { x: 3.0, y: 4.0 };
    let mut vec_i16 = Arr::from(vec![4i16, 5, 6]);
    let mut mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let mut b = false;
    let mut vec4 = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut i8 = 10i8;
    let mut i32 = 40i32;
    let mut vec_u8 = Arr::from(vec![3u8, 4, 5]);
    let ret = func(&mut f, &mut vec3, &mut ptr, &mut vec2, &mut vec_i16, &mut mat, &mut b, &mut vec4, &mut i8, &mut i32, &mut vec_u8);
    Str::from(format!(
        "{}|{}|{}|{:p}|{}|{}|{}|{}|{}|{}|{}|{}",
        ret,
        f,
        format_vec3(&vec3),
        ptr as *const(),
        format_vec2(&vec2),
        ArrFormat::format(&vec_i16),
        format_mat(&mat),
        format_bool(b),
        format_vec4(&vec4),
        i8,
        i32,
        ArrFormat::format(&vec_u8)
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc28(func: Func28) -> Str {
    let mut ptr = 1usize;
    let mut u16 = 20u16;
    let mut vec_u32 = Arr::from(vec![4u32, 5, 6]);
    let mut mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let mut f = 2.71f32;
    let mut vec4 = Vec4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut str = Str::from("New example string");
    let mut vec_u64 = Arr::from(vec![400u64, 500, 600]);
    let mut i64 = 987654321i64;
    let mut b = false;
    let mut vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut vec_f = Arr::from(vec![4.0f32, 5.0, 6.0]);
    let ret = func(&mut ptr, &mut u16, &mut vec_u32, &mut mat, &mut f, &mut vec4, &mut str, &mut vec_u64, &mut i64, &mut b, &mut vec3, &mut vec_f);
    Str::from(format!(
        "{}|{:p}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        ret,
        ptr as *const(),
        u16,
        ArrFormat::format(&vec_u32),
        format_mat(&mat),
        f,
        format_vec4(&vec4),
        str.to_string(),
        ArrFormat::format(&vec_u64),
        i64,
        format_bool(b),
        format_vec3(&vec3),
        ArrFormat::format(&vec_f)
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc29(func: Func29) -> Str {
    let mut vec4 = Vec4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
    let mut i32 = 99i32;
    let mut vec_i8 = Arr::from(vec![4i8, 5, 6]);
    let mut d = 2.71f64;
    let mut b = false;
    let mut i8 = 10i8;
    let mut vec_u16 = Arr::from(vec![4u16, 5, 6]);
    let mut f = 3.21f32;
    let mut str = Str::from("Yet another example string");
    let mut mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let mut u64 = 200u64;
    let mut vec3 = Vec3 { x: 5.0, y: 6.0, z: 7.0 };
    let mut vec_i64 = Arr::from(vec![2000i64, 3000, 4000]);
    let ret = func(&mut vec4, &mut i32, &mut vec_i8, &mut d, &mut b, &mut i8, &mut vec_u16, &mut f, &mut str, &mut mat, &mut u64, &mut vec3, &mut vec_i64);
    Str::from(format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        ArrFormat::format(&ret),
        format_vec4(&vec4),
        i32,
        ArrFormat::format(&vec_i8),
        d,
        format_bool(b),
        i8,
        ArrFormat::format(&vec_u16),
        f,
        str.to_string(),
        format_mat(&mat),
        u64,
        format_vec3(&vec3),
        ArrFormat::format(&vec_i64)
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc30(func: Func30) -> Str {
    let mut ptr = 1usize;
    let mut vec4 = Vec4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
    let mut i64 = 987654321i64;
    let mut vec_u32 = Arr::from(vec![4u32, 5, 6]);
    let mut b = false;
    let mut str = Str::from("Updated String for Func30");
    let mut vec3 = Vec3 { x: 5.0, y: 6.0, z: 7.0 };
    let mut vec_u8 = Arr::from(vec![1u8, 2, 3]);
    let mut f = 5.67f32;
    let mut vec2 = Vec2 { x: 3.0, y: 4.0 };
    let mut mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let mut i8 = 10i8;
    let mut vec_f = Arr::from(vec![4.0f32, 5.0, 6.0]);
    let mut d = 8.90f64;
    let ret = func(&mut ptr, &mut vec4, &mut i64, &mut vec_u32, &mut b, &mut str, &mut vec3, &mut vec_u8, &mut f, &mut vec2, &mut mat, &mut i8, &mut vec_f, &mut d);
    Str::from(format!(
        "{}|{:p}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        ret,
        ptr as *const(),
        format_vec4(&vec4),
        i64,
        ArrFormat::format(&vec_u32),
        format_bool(b),
        str.to_string(),
        format_vec3(&vec3),
        ArrFormat::format(&vec_u8),
        f,
        format_vec2(&vec2),
        format_mat(&mat),
        i8,
        ArrFormat::format(&vec_f),
        d
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc31(func: Func31) -> Str {
    let mut ch = b'B' as i8;
    let mut u32 = 200u32;
    let mut vec_u64 = Arr::from(vec![4u64, 5, 6]);
    let mut vec4 = Vec4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
    let mut str = Str::from("Updated String for Func31");
    let mut b = true;
    let mut i64 = 987654321i64;
    let mut vec2 = Vec2 { x: 3.0, y: 4.0 };
    let mut i8 = 10i8;
    let mut u16 = 20u16;
    let mut vec_i16 = Arr::from(vec![4i16, 5, 6]);
    let mut mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let mut vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut f = 5.67f32;
    let mut vec_d = Arr::from(vec![4.0f64, 5.0, 6.0]);
    let ret = func(&mut ch, &mut u32, &mut vec_u64, &mut vec4, &mut str, &mut b, &mut i64, &mut vec2, &mut i8, &mut u16, &mut vec_i16, &mut mat, &mut vec3, &mut f, &mut vec_d);
    Str::from(format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        format_vec3(&ret),
        ch as u8 as char,
        u32,
        ArrFormat::format(&vec_u64),
        format_vec4(&vec4),
        str.to_string(),
        format_bool(b),
        i64,
        format_vec2(&vec2),
        i8,
        u16,
        ArrFormat::format(&vec_i16),
        format_mat(&mat),
        format_vec3(&vec3),
        f,
        ArrFormat::format(&vec_d)
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc32(func: Func32) -> Str {
    let mut i32 = 30i32;
    let mut u16 = 20u16;
    let mut vec_i8 = Arr::from(vec![4i8, 5, 6]);
    let mut vec4 = Vec4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
    let mut ptr = 1usize;
    let mut vec_u32 = Arr::from(vec![4u32, 5, 6]);
    let mut mat = Mat4x4 { m: [[0.0; 4]; 4] };
    let mut u64 = 200u64;
    let mut str = Str::from("Updated String for Func32");
    let mut i64 = 987654321i64;
    let mut vec2 = Vec2 { x: 3.0, y: 4.0 };
    let mut vec_i8_2 = Arr::from(vec![7i8, 8, 9]);
    let mut b = false;
    let mut vec3 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut u8 = 128u8;
    let mut vec_c16 = Arr::from(vec!['D' as u16, 'E' as u16, 'F' as u16]);
    let _ret = func(&mut i32, &mut u16, &mut vec_i8, &mut vec4, &mut ptr, &mut vec_u32, &mut mat, &mut u64, &mut str, &mut i64, &mut vec2, &mut vec_i8_2, &mut b, &mut vec3, &mut u8, &mut vec_c16);
    Str::from(format!(
        "{}|{}|{}|{}|{:p}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        i32,
        u16,
        ArrFormat::format(&vec_i8),
        format_vec4(&vec4),
        ptr as *const(),
        ArrFormat::format(&vec_u32),
        format_mat(&mat),
        u64,
        str.to_string(),
        i64,
        format_vec2(&vec2),
        ArrFormat::format(&vec_i8_2),
        format_bool(b),
        format_vec3(&vec3),
        u8,
        ArrFormat::format(&vec_c16)
    ))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc33(func: Func33) -> Str {
    let mut variant = Var::new(&Any::Int32(30));
    func(&mut variant);
    match variant.get() {
        Any::String(v) => Str::from(v),
        _ => Str::from("NA"),
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncEnum(func: FuncEnum) -> Str {
    let p1 = Example::Forth;
    let mut p2 = Arr::from_slice(&[Example::Forth, Example::Forth, Example::Forth]);
    let ret = func(p1, &mut p2);
    Str::from(format!("{}|{}", ArrFormat::format(&ret), ArrFormat::format(&p2)))
}


// ============================================================================
// Mock Functions for ReverseCall Tests
// ============================================================================

extern "C" fn mock_void() {}
extern "C" fn mock_bool() -> bool { true }
extern "C" fn mock_char8() -> i8 { b'A' as i8 }
extern "C" fn mock_char16() -> u16 { 'Z' as u16 }
extern "C" fn mock_int8() -> i8 { 10 }
extern "C" fn mock_int16() -> i16 { 100 }
extern "C" fn mock_int32() -> i32 { 1000 }
extern "C" fn mock_int64() -> i64 { 10000 }
extern "C" fn mock_uint8() -> u8 { 20 }
extern "C" fn mock_uint16() -> u16 { 200 }
extern "C" fn mock_uint32() -> u32 { 2000 }
extern "C" fn mock_uint64() -> u64 { 20000 }
extern "C" fn mock_ptr() -> usize { 0 }
extern "C" fn mock_float() -> f32 { 3.14 }
extern "C" fn mock_double() -> f64 { 6.28 }
//extern "C" fn mock_function() -> usize { 0 }
extern "C" fn mock_string() -> Str { Str::from("Test string") }
extern "C" fn mock_any() -> Var { Var::from(Any::Char16('A' as u16)) }

extern "C" fn mock_bool_vector() -> Arr<bool> { Arr::from(vec![true, false]) }
extern "C" fn mock_char8_vector() -> Arr<i8> { Arr::from(vec![b'A' as i8, b'B' as i8]) }
extern "C" fn mock_char16_vector() -> Arr<u16> { Arr::from(vec!['A' as u16, 'B' as u16]) }
extern "C" fn mock_int8_vector() -> Arr<i8> { Arr::from(vec![10, 20]) }
extern "C" fn mock_int16_vector() -> Arr<i16> { Arr::from(vec![100, 200]) }
extern "C" fn mock_int32_vector() -> Arr<i32> { Arr::from(vec![1000, 2000]) }
extern "C" fn mock_int64_vector() -> Arr<i64> { Arr::from(vec![10000, 20000]) }
extern "C" fn mock_uint8_vector() -> Arr<u8> { Arr::from(vec![20, 30]) }
extern "C" fn mock_uint16_vector() -> Arr<u16> { Arr::from(vec![200, 300]) }
extern "C" fn mock_uint32_vector() -> Arr<u32> { Arr::from(vec![2000, 3000]) }
extern "C" fn mock_uint64_vector() -> Arr<u64> { Arr::from(vec![20000, 30000]) }
extern "C" fn mock_ptr_vector() -> Arr<usize> { Arr::from(vec![0, 1]) }
extern "C" fn mock_float_vector() -> Arr<f32> { Arr::from(vec![1.1, 2.2]) }
extern "C" fn mock_double_vector() -> Arr<f64> { Arr::from(vec![3.3, 4.4]) }
extern "C" fn mock_string_vector() -> Arr<Str> {
    Arr::from(vec![Str::from("Hello"), Str::from("World")])
}
extern "C" fn mock_any_vector() -> Arr<Var> {
    Arr::from(vec![
        Any::from("Hello"),
        Any::from(3.14f32),
        Any::from(6.28f64),
        Any::from(1i32),
        Any::from(0xDEADBEAF_usize),
    ])
}

extern "C" fn mock_vec2_vector() -> Arr<Vec2> {
    Arr::from(vec![
        Vec2 { x: 0.5, y: -1.2 },
        Vec2 { x: 3.4, y: 7.8 },
        Vec2 { x: -6.7, y: 2.3 },
        Vec2 { x: 8.9, y: -4.5 },
        Vec2 { x: 0.0, y: 0.0 },
    ])
}

extern "C" fn mock_vec3_vector() -> Arr<Vec3> {
    Arr::from(vec![
        Vec3 { x: 2.1, y: 3.2, z: 4.3 },
        Vec3 { x: -5.4, y: 6.5, z: -7.6 },
        Vec3 { x: 8.7, y: 9.8, z: 0.1 },
        Vec3 { x: 1.2, y: -3.3, z: 4.4 },
        Vec3 { x: -5.5, y: 6.6, z: -7.7 },
    ])
}

extern "C" fn mock_vec4_vector() -> Arr<Vec4> {
    Arr::from(vec![
        Vec4 { x: 0.1, y: 1.2, z: 2.3, w: 3.4 },
        Vec4 { x: -4.5, y: 5.6, z: 6.7, w: -7.8 },
        Vec4 { x: 8.9, y: -9.0, z: 10.1, w: -11.2 },
        Vec4 { x: 12.3, y: 13.4, z: 14.5, w: 15.6 },
        Vec4 { x: -16.7, y: 17.8, z: 18.9, w: -19.0 },
    ])
}

extern "C" fn mock_mat4x4_vector() -> Arr<Mat4x4> {
    Arr::from(vec![
        Mat4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        },
        Mat4x4 {
            m: [
                [0.5, 1.0, 1.5, 2.0],
                [2.5, 3.0, 3.5, 4.0],
                [4.5, 5.0, 5.5, 6.0],
                [6.5, 7.0, 7.5, 8.0],
            ]
        },
        Mat4x4 {
            m: [
                [-1.0, -2.0, -3.0, -4.0],
                [-5.0, -6.0, -7.0, -8.0],
                [-9.0, -10.0, -11.0, -12.0],
                [-13.0, -14.0, -15.0, -16.0],
            ]
        },
        Mat4x4 {
            m: [
                [1.1, 2.2, 3.3, 4.4],
                [5.5, 6.6, 7.7, 8.8],
                [9.9, 10.0, 11.1, 12.2],
                [13.3, 14.4, 15.5, 16.6],
            ]
        },
    ])
}

extern "C" fn mock_vec2() -> Vec2 { Vec2 { x: 1.0, y: 2.0 } }
extern "C" fn mock_vec3() -> Vec3 { Vec3 { x: 1.0, y: 2.0, z: 3.0 } }
extern "C" fn mock_vec4() -> Vec4 { Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 } }
extern "C" fn mock_mat4x4() -> Mat4x4 {
    Mat4x4 {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]
    }
}

// Mock implementations for 1+ parameter functions
extern "C" fn mock_func1(v: &Vec3) -> i32 {
    let _buffer = format!("{}{}{}", v.x, v.y, v.z);
    (v.x + v.y + v.z) as i32
}

extern "C" fn mock_func2(a: f32, b: i64) -> i8 {
    let _buffer = format!("{}{}", a, b);
    b'&' as i8
}

extern "C" fn mock_func3(p: usize, v: &Vec4, s: &Str) {
    let _buffer = format!("{}{}{}{}{}{}", p, v.x, v.y, v.z, v.w, s.as_str());
}

extern "C" fn mock_func4(flag: bool, u: i32, c: u16, _m: &Mat4x4) -> Vec4 {
    let _buffer = format!("{}{}{}", flag, u, c);
    Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 }
}

extern "C" fn mock_func5(i: i8, v: &Vec2, p: usize, d: f64, vec: &Arr<u64>) -> bool {
    let _buffer = format!("{}{}{}{}{}{}", i, v.x, v.y, p, d, vec.len());
    true
}

extern "C" fn mock_func6(s: &Str, f: f32, vec: &Arr<f32>, i: i16, u_vec: &Arr<u8>, p: usize) -> i64 {
    let _buffer = format!("{}{}{}{}{}{}", s.as_str(), f, vec.len(), i, u_vec.len(), p);
    (f + i as f32) as i64
}

extern "C" fn mock_func7(vec: &Arr<i8>, u: u16, c: u16, u_vec: &Arr<u32>, v: &Vec4, flag: bool, l: u64) -> f64 {
    let _buffer = format!("{}{}{}{}{}{}{}{}{}{}", vec.len(), u, c, u_vec.len(), v.x, v.y, v.z, v.w, flag, l);
    3.14
}

extern "C" fn mock_func8(_v: &Vec3, _u_vec: &Arr<u32>, _i: i16, _flag: bool, _v4: &Vec4, _c_vec: &Arr<u16>, _c: u16, _a: i32) -> Mat4x4 {
    Mat4x4 { m: [[0.0; 4]; 4] }
}

extern "C" fn mock_func9(_f: f32, _v: &Vec2, _i_vec: &Arr<i8>, _l: u64, _flag: bool, _s: &Str, _v4: &Vec4, _i: i16, _p: usize) {
}

extern "C" fn mock_func10(_v4: &Vec4, _m: &Mat4x4, _u_vec: &Arr<u32>, _l: u64, _c_vec: &Arr<i8>, _a: i32, _flag: bool, _v: &Vec2, _i: i64, _d: f64) -> u32 {
    42
}

extern "C" fn mock_func11(b_vec: &Arr<bool>, c: u16, u: u8, d: f64, v3: &Vec3, i_vec: &Arr<i8>, i: i64, u16: u16, f: f32, v: &Vec2, u32: u32) -> usize {
    let _buffer = format!("{}{}{}{}{}{}{}{}{}{}{}", b_vec.len(), c, u, d, v3.x, i_vec.len(), i, u16, f, v.x, u32);
    0
}

extern "C" fn mock_func12(p: usize, d_vec: &Arr<f64>, u: u32, d: f64, flag: bool, a: i32, i: i8, l: u64, f: f32, p_vec: &Arr<usize>, i64: i64, c: i8) -> bool {
    let _buffer = format!("{}{}{}{}{}{}{}{}{}{}{}{}", p, d_vec.len(), u, d, flag, a, i, l, f, p_vec.len(), i64, c);
    false
}

extern "C" fn mock_func13(_i64: i64, _c_vec: &Arr<i8>, _u16: u16, _f: f32, _b_vec: &Arr<bool>, _v4: &Vec4, _s: &Str, _a: i32, _v3: &Vec3, _p: usize, _v2: &Vec2, _u8_vec: &Arr<u8>, _i16: i16) -> Str {
    Str::from("Dummy String")
}

extern "C" fn mock_func14(_c_vec: &Arr<i8>, _u_vec: &Arr<u32>, _m: &Mat4x4, _flag: bool, _c: u16, _a: i32, _f_vec: &Arr<f32>, _u16: u16, _u8_vec: &Arr<u8>, _i8: i8, _v3: &Vec3, _v4: &Vec4, _d: f64, _p: usize) -> Arr<Str> {
    Arr::from(vec![Str::from("String1"), Str::from("String2")])
}

extern "C" fn mock_func15(_i_vec: &Arr<i16>, _m: &Mat4x4, _v4: &Vec4, _p: usize, _l: u64, _u_vec: &Arr<u32>, _flag: bool, _f: f32, _c_vec: &Arr<u16>, _u: u8, _a: i32, _v2: &Vec2, _u16: u16, _d: f64, _u8_vec: &Arr<u8>) -> i16 {
    257
}

extern "C" fn mock_func16(b_vec: &Arr<bool>, i16: i16, i_vec: &Arr<i8>, v4: &Vec4, m: &Mat4x4, v2: &Vec2, u_vec: &Arr<u64>, c_vec: &Arr<i8>, s: &Str, i64: i64, u32_vec: &Arr<u32>, v3: &Vec3, f: f32, d: f64, i8: i8, u16: u16) -> usize {
    let _buffer = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", i16, b_vec.len(), i_vec.len(), v4.x, v4.y, v4.z, v4.w, m.m[3][3], v2.x, u_vec.len(), c_vec.len(), s.as_str(), i64, u32_vec.len(), v3.x, f, d, i8, u16);
    0
}

extern "C" fn mock_func17(r: &mut i32) {
    *r += 10;
}

extern "C" fn mock_func18(i8: &mut i8, i16: &mut i16) -> Vec2 {
    *i8 = 5;
    *i16 = 10;
    Vec2 { x: *i8 as f32, y: *i16 as f32 }
}

extern "C" fn mock_func19(u32: &mut u32, v3: &mut Vec3, u_vec: &mut Arr<u32>) {
    *u32 = 42;
    *v3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    u_vec.set(&[1, 2, 3]);
}

extern "C" fn mock_func20(c: &mut u16, v4: &mut Vec4, u_vec: &mut Arr<u64>, ch: &mut i8) -> i32 {
    *c = 't' as u16;
    *v4 = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    u_vec.set(&[100, 200]);
    *ch = b'F' as i8;
    0
}

extern "C" fn mock_func21(m: &mut Mat4x4, i_vec: &mut Arr<i32>, v2: &mut Vec2, flag: &mut bool, d: &mut f64) -> f32 {
    *flag = true;
    *d = 3.14;
    *v2 = Vec2 { x: 1.0, y: 2.0 };
    *m = Mat4x4 {
        m: [
            [1.3, 0.6, 0.8, 0.5],
            [0.7, 1.1, 0.2, 0.4],
            [0.9, 0.3, 1.2, 0.7],
            [0.2, 0.8, 0.5, 1.0],
        ]
    };
    i_vec.set(&[1, 2, 3]);
    0.0
}

extern "C" fn mock_func22(p: &mut usize, u32: &mut u32, d_vec: &mut Arr<f64>, i16: &mut i16, s: &mut Str, v4: &mut Vec4) -> u64 {
    *p = 0;
    *u32 = 99;
    *i16 = 123;
    s.set("Hello");
    *v4 = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    d_vec.set(&[1.1, 2.2, 3.3]);
    0
}

extern "C" fn mock_func23(u64: &mut u64, v2: &mut Vec2, i_vec: &mut Arr<i16>, c: &mut u16, f: &mut f32, i8: &mut i8, u8_vec: &mut Arr<u8>) {
    *u64 = 50;
    *f = 1.5;
    *i8 = -1;
    *v2 = Vec2 { x: 3.0, y: 4.0 };
    u8_vec.set(&[1, 2, 3]);
    *c = 0x2164; // Roman numeral V character
    i_vec.set(&[1, 2, 3, 4]);
}

extern "C" fn mock_func24(c_vec: &mut Arr<i8>, i64: &mut i64, u8_vec: &mut Arr<u8>, v4: &mut Vec4, u64: &mut u64, p_vec: &mut Arr<usize>, d: &mut f64, v_vec: &mut Arr<usize>) -> Mat4x4 {
    *i64 = 64;
    *d = 2.71;
    *v4 = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    c_vec.set(&[b'a' as i8, b'b' as i8, b'c' as i8]);
    u8_vec.set(&[5, 6, 7]);
    p_vec.set(&[0]);
    v_vec.set(&[1, 1, 2, 2]);
    *u64 = 0xffffffff;
    Mat4x4 { m: [[0.0; 4]; 4] }
}

extern "C" fn mock_func25(i32: &mut i32, p_vec: &mut Arr<usize>, flag: &mut bool, u8: &mut u8, s: &mut Str, v3: &mut Vec3, i64: &mut i64, v4: &mut Vec4, u16: &mut u16) -> f64 {
    *flag = false;
    *i32 = 100;
    *u8 = 250;
    *v3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    *v4 = Vec4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 };
    s.set("MockFunc25");
    p_vec.set(&[0]);
    *i64 = 1337;
    *u16 = 64222;
    0.0
}

extern "C" fn mock_func26(c: &mut u16, v2: &mut Vec2, m: &mut Mat4x4, f_vec: &mut Arr<f32>, i16: &mut i16, u64: &mut u64, u32: &mut u32, u16_vec: &mut Arr<u16>, p: &mut usize, flag: &mut bool) -> i8 {
    *c = 'Z' as u16;
    *flag = true;
    *v2 = Vec2 { x: 2.0, y: 3.0 };
    *m = Mat4x4 {
        m: [
            [0.9, 0.2, 0.4, 0.8],
            [0.1, 1.0, 0.6, 0.3],
            [0.7, 0.5, 0.2, 0.9],
            [0.3, 0.4, 1.5, 0.1],
        ]
    };
    f_vec.set(&[1.1, 2.2]);
    *u64 = 64;
    *u32 = 32;
    u16_vec.set(&[100, 200]);
    *i16 = 332;
    *p = 0xDEADBEAFDEADBEAF;
    b'A' as i8
}

extern "C" fn mock_func27(f: &mut f32, v3: &mut Vec3, p: &mut usize, v2: &mut Vec2, i16_vec: &mut Arr<i16>, m: &mut Mat4x4, flag: &mut bool, v4: &mut Vec4, i8: &mut i8, i32: &mut i32, u8_vec: &mut Arr<u8>) -> u8 {
    *f = 1.0;
    *v3 = Vec3 { x: -1.0, y: -2.0, z: -3.0 };
    *p = 0xDEADBEAFDEADBEAF;
    *v2 = Vec2 { x: -111.0, y: 111.0 };
    i16_vec.set(&[1, 2, 3, 4]);
    *m = Mat4x4 {
        m: [
            [1.0, 0.5, 0.3, 0.7],
            [0.8, 1.2, 0.6, 0.9],
            [1.5, 1.1, 0.4, 0.2],
            [0.3, 0.9, 0.7, 1.0],
        ]
    };
    *flag = true;
    *v4 = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    *i8 = 111;
    *i32 = 30;
    u8_vec.set(&[0, 0, 0, 0, 0, 0, 1, 0]);
    0
}

extern "C" fn mock_func28(ptr: &mut usize, u16: &mut u16, u32_vec: &mut Arr<u32>, m: &mut Mat4x4, f: &mut f32, v4: &mut Vec4, str: &mut Str, u64_vec: &mut Arr<u64>, i64: &mut i64, b: &mut bool, vec3: &mut Vec3, f_vec: &mut Arr<f32>) -> Str {
    *ptr = 0;
    *u16 = 65500;
    u32_vec.set(&[1, 2, 3, 4, 5, 7]);
    *m = Mat4x4 {
        m: [
            [1.4, 0.7, 0.2, 0.5],
            [0.3, 1.1, 0.6, 0.8],
            [0.9, 0.4, 1.3, 0.1],
            [0.6, 0.2, 0.7, 1.0],
        ]
    };
    *f = 5.5;
    *v4 = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    u64_vec.set(&[1, 2]);
    *i64 = 834748377834;
    *b = true;
    *vec3 = Vec3 { x: 10.0, y: 20.0, z: 30.0 };
    str.set("MockFunc28");
    f_vec.set(&[1.0, -1000.0, 2000.0]);
    Str::from("MockFunc28")
}

extern "C" fn mock_func29(v4: &mut Vec4, i32: &mut i32, i_vec: &mut Arr<i8>, d: &mut f64, flag: &mut bool, i8: &mut i8, u16_vec: &mut Arr<u16>, f: &mut f32, s: &mut Str, m: &mut Mat4x4, u64: &mut u64, v3: &mut Vec3, i64_vec: &mut Arr<i64>) -> Arr<Str> {
    *i32 = 30;
    *flag = true;
    *v4 = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    *d = 3.14;
    *i8 = 8;
    u16_vec.set(&[100, 200]);
    *f = 1.5;
    s.set("MockFunc29");
    *m = Mat4x4 {
        m: [
            [0.4, 1.0, 0.6, 0.3],
            [1.2, 0.8, 0.5, 0.9],
            [0.7, 0.3, 1.4, 0.6],
            [0.1, 0.9, 0.8, 1.3],
        ]
    };
    *u64 = 64;
    *v3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    i64_vec.set(&[1, 2, 3]);
    i_vec.set(&[127, 126, 125]);
    Arr::from(vec![Str::from("Example"), Str::from("MockFunc29")])
}

extern "C" fn mock_func30(p: &mut usize, v4: &mut Vec4, i64: &mut i64, u_vec: &mut Arr<u32>, flag: &mut bool, s: &mut Str, v3: &mut Vec3, u8_vec: &mut Arr<u8>, f: &mut f32, v2: &mut Vec2, m: &mut Mat4x4, i8: &mut i8, vec: &mut Arr<f32>, d: &mut f64) -> i32 {
    *flag = false;
    *f = 1.1;
    *i64 = 1000;
    *v2 = Vec2 { x: 3.0, y: 4.0 };
    *v4 = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    s.set("MockFunc30");
    *p = 0;
    u_vec.set(&[100, 200]);
    *m = Mat4x4 {
        m: [
            [0.5, 0.3, 1.0, 0.7],
            [1.1, 0.9, 0.6, 0.4],
            [0.2, 0.8, 1.5, 0.3],
            [0.7, 0.4, 0.9, 1.0],
        ]
    };
    *i8 = 8;
    vec.set(&[1.0, 1.0, 2.0, 2.0]);
    *d = 2.718;
    *v3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    u8_vec.set(&[255, 0, 255, 200, 100, 200]);
    42
}

extern "C" fn mock_func31(c: &mut i8, u32: &mut u32, u_vec: &mut Arr<u64>, v4: &mut Vec4, s: &mut Str, flag: &mut bool, i64: &mut i64, v2: &mut Vec2, i8: &mut i8, u16: &mut u16, i_vec: &mut Arr<i16>, m: &mut Mat4x4, v3: &mut Vec3, f: &mut f32, v4_vec: &mut Arr<f64>) -> Vec3 {
    *u32 = 12345;
    *flag = true;
    *v3 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    *c = b'C' as i8;
    *v4 = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    s.set("MockFunc31");
    *i64 = 123456789;
    *v2 = Vec2 { x: 5.0, y: 6.0 };
    *i8 = 7;
    *u16 = 255;
    i_vec.set(&[1, 2]);
    *m = Mat4x4 {
        m: [
            [0.8, 0.5, 1.2, 0.3],
            [1.0, 0.7, 0.4, 0.6],
            [0.9, 0.2, 0.5, 1.4],
            [0.6, 0.8, 1.1, 0.7],
        ]
    };
    v4_vec.set(&[1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0]);
    u_vec.set(&[1, 2, 3, 4, 5]);
    *f = -1.0;
    Vec3 { x: 1.0, y: 1.5, z: 3.0 }
}

extern "C" fn mock_func32(i32: &mut i32, u16: &mut u16, i_vec: &mut Arr<i8>, v4: &mut Vec4, p: &mut usize, u_vec: &mut Arr<u32>, m: &mut Mat4x4, u64: &mut u64, s: &mut Str, i64: &mut i64, v2: &mut Vec2, u8_vec: &mut Arr<i8>, flag: &mut bool, v3: &mut Vec3, u8: &mut u8, c_vec: &mut Arr<u16>) -> f64 {
    *i32 = 42;
    *u16 = 255;
    *flag = false;
    *v2 = Vec2 { x: 2.5, y: 3.5 };
    u8_vec.set(&[1, 2, 3, 4, 5, 9]);
    *v4 = Vec4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 };
    s.set("MockFunc32");
    *p = 0;
    *m = Mat4x4 {
        m: [
            [1.0, 0.4, 0.3, 0.9],
            [0.7, 1.2, 0.5, 0.8],
            [0.2, 0.6, 1.1, 0.4],
            [0.9, 0.3, 0.8, 1.5],
        ]
    };
    *u64 = 123456789;
    u_vec.set(&[100, 200]);
    *i64 = 1000;
    *v3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    *u8 = 8;
    c_vec.set(&['a' as u16, 'b' as u16, 'c' as u16]);
    i_vec.set(&[0, 1]);
    1.0
}

extern "C" fn mock_func33(variant: &mut Var) {
    variant.set(&Any::from("MockFunc33"));
}

extern "C" fn mock_func_enum(p1: MasterExample, p2: &mut Arr<MasterExample>) -> Arr<MasterExample> {
    p2.set(&[MasterExample::First, MasterExample::Second, MasterExample::Third]);
    Arr::from_slice(&[p1, MasterExample::Forth])
}

#[cfg(feature = "verbose")]
fn log(message: &str) {
    println!("{}", message);
}
#[cfg(not(feature = "verbose"))]
fn log(_message: &str) {}

pub fn basic_lifecycle() -> Str {
    log("TEST 1: Basic Lifecycle");
    log("_______________________");

    let initial_alive = ResourceHandle::GetAliveCount();
    let initial_created = ResourceHandle::GetTotalCreated();

    {
        let resource = match ResourceHandle::new_ResourceHandleCreate(1, &Str::from("Test1")) {
            Ok(r) => r,
            Err(_) => return Str::from("false"),
        };

        log(&format!("v Created ResourceHandle ID: {}", resource.GetId().unwrap()));
        log(&format!(
            "v Alive count increased: {}",
            ResourceHandle::GetAliveCount()
        ));
    }

    let final_alive = ResourceHandle::GetAliveCount();
    let final_created = ResourceHandle::GetTotalCreated();
    let final_destroyed = ResourceHandle::GetTotalDestroyed();

    log(&format!("v Destructor called, alive count: {}", final_alive));
    log(&format!("v Total created: {}", final_created - initial_created));
    log(&format!("v Total destroyed: {}", final_destroyed));

    if final_alive == initial_alive && final_created == final_destroyed {
        log("v TEST 1 PASSED: Lifecycle working correctly\n");
        Str::from("true")
    } else {
        log("x TEST 1 FAILED: Lifecycle mismatch!\n");
        Str::from("false")
    }
}

pub fn state_management() -> Str {
    log("TEST 2: State Management");
    log("________________________");

    let resource = match ResourceHandle::new_ResourceHandleCreate(2, &Str::from("StateTest")) {
        Ok(r) => r,
        Err(_) => return Str::from("false"),
    };

    if resource.IncrementCounter().is_err() { return Str::from("false"); }
    if resource.IncrementCounter().is_err() { return Str::from("false"); }
    if resource.IncrementCounter().is_err() { return Str::from("false"); }

    let counter = resource.GetCounter().unwrap_or_default();
    log(&format!("v Counter incremented 3 times: {}", counter));

    if resource.SetName(&Str::from("StateTestModified")).is_err() { return Str::from("false"); }
    let new_name = resource.GetName().unwrap_or_else(|_| Str::from(""));

    log(&format!("v Name changed to: {}", new_name));

    if resource.AddData(1.1f32).is_err() { return Str::from("false"); }
    if resource.AddData(2.2f32).is_err() { return Str::from("false"); }
    if resource.AddData(3.3f32).is_err() { return Str::from("false"); }

    let data = resource.GetData().unwrap_or_else(|_| Arr::new());
    log(&format!("v Added {} data points", data.len()));

    if counter == 3 && new_name == "StateTestModified" && data.len() == 3 {
        log("v TEST 2 PASSED: State management working\n");
        Str::from("true")
    } else {
        log("x TEST 2 FAILED: State not preserved!\n");
        Str::from("false")
    }
}

pub fn multiple_instances() -> Str {
    log("TEST 3: Multiple Instances");
    log("__________________________");

    let before_alive = ResourceHandle::GetAliveCount();

    {
        let r1 = ResourceHandle::new_ResourceHandleCreate(10, &Str::from("Instance1"));
        let r2 = ResourceHandle::new_ResourceHandleCreate(20, &Str::from("Instance2"));
        let r3 = ResourceHandle::new_ResourceHandleCreateDefault();

        let r1 = match r1 { Ok(v) => v, Err(_) => return Str::from("false") };
        let r2 = match r2 { Ok(v) => v, Err(_) => return Str::from("false") };
        let r3 = match r3 { Ok(v) => v, Err(_) => return Str::from("false") };

        let during_alive = ResourceHandle::GetAliveCount();
        log(&format!("v Created 3 instances, alive: {}", during_alive));
        log(&format!(
            "v R1 ID: {}, R2 ID: {}, R3 ID: {}",
            r1.GetId().unwrap_or_default(),
            r2.GetId().unwrap_or_default(),
            r3.GetId().unwrap_or_default()
        ));

        if during_alive - before_alive == 3 {
            log("v All 3 instances tracked correctly");
        }
    }

    let after_alive = ResourceHandle::GetAliveCount();

    if after_alive == before_alive {
        log("v TEST 3 PASSED: All instances destroyed properly\n");
        Str::from("true")
    } else {
        log(&format!("x TEST 3 FAILED: Leak detected! Before: {}, After: {}\n", before_alive, after_alive));
        Str::from("false")
    }
}

pub fn counter_without_destructor() -> Str {
    log("TEST 4: Counter (No Destructor)");
    log("________________________________");

    let counter = match Counter::new_CounterCreate(100) {
        Ok(c) => c,
        Err(_) => return Str::from("false"),
    };

    log(&format!("v Created Counter with value: {}", counter.GetValue().unwrap_or_default()));

    counter.Increment().ok();
    counter.Increment().ok();
    counter.Add(50).ok();

    let value = counter.GetValue().unwrap_or_default();
    log(&format!("v After operations, value: {}", value));

    let is_positive = counter.IsPositive().unwrap_or(false);
    log(&format!("v Is positive: {}", is_positive));

    if value == 152 && is_positive {
        log("v TEST 4 PASSED: Counter operations working\n");
        Str::from("true")
    } else {
        log("x TEST 4 FAILED: Counter operations incorrect\n");
        Str::from("false")
    }
}

pub fn static_methods() -> Str {
    log("TEST 5: Static Methods");
    log("______________________");

    let alive = ResourceHandle::GetAliveCount();
    let created = ResourceHandle::GetTotalCreated();
    let destroyed = ResourceHandle::GetTotalDestroyed();
    log(&format!(
        "v ResourceHandle stats - Alive: {}, Created: {}, Destroyed: {}",
        alive, created, destroyed
    ));

    let cmp1 = Counter::Compare(100, 50);
    let cmp2 = Counter::Compare(50, 100);
    let cmp3 = Counter::Compare(50, 50);
    log(&format!("v Counter.Compare(100, 50) = {} (expected 1)", cmp1));
    log(&format!("v Counter.Compare(50, 100) = {} (expected -1)", cmp2));
    log(&format!("v Counter.Compare(50, 50) = {} (expected 0)", cmp3));

    let values = Arr::from_slice(&[1i64, 2, 3, 4, 5]);
    let sum = Counter::Sum(&values);
    log(&format!("v Counter.Sum([1,2,3,4,5]) = {} (expected 15)", sum));

    if cmp1 == 1 && cmp2 == -1 && cmp3 == 0 && sum == 15 {
        log("v TEST 5 PASSED: Static methods working\n");
        Str::from("true")
    } else {
        log("x TEST 5 FAILED: Static methods incorrect\n");
        Str::from("false")
    }
}

pub fn memory_leak_detection() -> Str {
    log("TEST 6: Memory Leak Detection");
    log("______________________________");

    let before_alive = ResourceHandle::GetAliveCount();

    {
        let leaked = match ResourceHandle::new_ResourceHandleCreate(999, &Str::from("IntentionalLeak")) {
            Ok(r) => r,
            Err(_) => return Str::from("false"),
        };
        log(&format!("v Created resource ID: {}", leaked.GetId().unwrap_or_default()));
        // leaked goes out of scope here
    }

    let after_alive = ResourceHandle::GetAliveCount();

    log(&format!("v Before leak test: {} alive", before_alive));
    log(&format!("v After release: {} alive", after_alive));

    if after_alive == before_alive {
        log("v TEST 6 PASSED: Destructor cleaned up leaked resource\n");
        Str::from("true")
    } else {
        log("x TEST 6 FAILED: Resource still alive (FATAL)\n");
        Str::from("false")
    }
}

pub fn exception_handling() -> Str {
    log("TEST 7: Exception Handling");
    log("__________________________");

    let mut resource = match ResourceHandle::new_ResourceHandleCreate(777, &Str::from("ExceptionTest")) {
        Ok(r) => r,
        Err(_) => return Str::from("false"),
    };

    resource.reset();

    match resource.GetId() {
        Ok(_) => {
            log("x TEST 7 FAILED: No exception thrown!\n");
            Str::from("false")
        }
        Err(_) => {
            log("v Caught expected error from GetId on reset resource");
            log("v TEST 7 PASSED: Exception handling working\n");
            Str::from("true")
        }
    }
}

pub fn ownership_transfer() -> Str {
    log("TEST 8: Ownership Transfer (get + release)");
    log("─────────────────────────────────────────");

    let initial_alive = ResourceHandle::GetAliveCount();
    let _initial_created = ResourceHandle::GetTotalCreated();

    let mut resource = match ResourceHandle::new_ResourceHandleCreate(42, &Str::from("OwnershipTest")) {
        Ok(r) => r,
        Err(_) => return Str::from("false"),
    };

    log(&format!("v Created ResourceHandle ID: {}", resource.GetId().unwrap_or_default()));

    let wrapper = resource.get();
    log(&format!("v get() returned internal wrapper: {}", wrapper));

    let handle = resource.release();
    log(&format!("v release() returned handle: {}", handle));

    if wrapper != handle {
        log("x TEST 8 FAILED: get() did not return internal wrapper");
        return Str::from("false");
    }

    if resource.GetId().is_ok() {
        log("x TEST 8 FAILED: ResourceHandle still accessible after release()");
        return Str::from("false");
    } else {
        log("v ResourceHandle is invalid after release()");
    }

    let alive_after_release = ResourceHandle::GetAliveCount();
    if alive_after_release != initial_alive + 1 {
        log(&format!(
            "x TEST 8 FAILED: Alive count mismatch after release. Expected {}, got {}",
            initial_alive + 1,
            alive_after_release
        ));
        return Str::from("false");
    }

    // Destroy external handle
    ResourceHandleDestroy(handle);

    log("v TEST 8 PASSED: Ownership transfer working correctly\n");
    Str::from("true")
}

// ============================================================================
// ReverseCall Function
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ReverseCall(test: &Str) {
    let test_name = test.as_str();

    match test_name {
        "NoParamReturnVoid" => {
            NoParamReturnVoidCallback();
        }
        "NoParamReturnBool" => {
            let result = NoParamReturnBoolCallback();
            let result_str = format_bool(result);
            ReverseReturn(&Str::from(result_str));
        }
        "NoParamReturnChar8" => {
            let result = NoParamReturnChar8Callback();
            ReverseReturn(&Str::from(format!("{}", result as u8 as char)));
        }
        "NoParamReturnChar16" => {
            let result = NoParamReturnChar16Callback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnInt8" => {
            let result = NoParamReturnInt8Callback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnInt16" => {
            let result = NoParamReturnInt16Callback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnInt32" => {
            let result = NoParamReturnInt32Callback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnInt64" => {
            let result = NoParamReturnInt64Callback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnUInt8" => {
            let result = NoParamReturnUInt8Callback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnUInt16" => {
            let result = NoParamReturnUInt16Callback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnUInt32" => {
            let result = NoParamReturnUInt32Callback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnUInt64" => {
            let result = NoParamReturnUInt64Callback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnPointer" => {
            let result = NoParamReturnPointerCallback();
            ReverseReturn(&Str::from(format!("{:p}", result as *const ())));
        }
        "NoParamReturnFloat" => {
            let result = NoParamReturnFloatCallback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnDouble" => {
            let result = NoParamReturnDoubleCallback();
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "NoParamReturnFunction" => {
            let result = NoParamReturnFunctionCallback();
            let res;
            unsafe { res = result(); }
            ReverseReturn(&Str::from(res.to_string()));
        }
        "NoParamReturnString" => {
            let result = NoParamReturnStringCallback();
            ReverseReturn(&result);
        }
        "NoParamReturnAny" => {
            let result = NoParamReturnAnyCallback();
            ReverseReturn(&Str::from(format!("{:?}", result.get())));
        }
        "NoParamReturnArrayBool" => {
            let result = NoParamReturnArrayBoolCallback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayChar8" => {
            let result = NoParamReturnArrayChar8Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayChar16" => {
            let result = NoParamReturnArrayChar16Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayInt8" => {
            let result = NoParamReturnArrayInt8Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayInt16" => {
            let result = NoParamReturnArrayInt16Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayInt32" => {
            let result = NoParamReturnArrayInt32Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayInt64" => {
            let result = NoParamReturnArrayInt64Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayUInt8" => {
            let result = NoParamReturnArrayUInt8Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayUInt16" => {
            let result = NoParamReturnArrayUInt16Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayUInt32" => {
            let result = NoParamReturnArrayUInt32Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayUInt64" => {
            let result = NoParamReturnArrayUInt64Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayPointer" => {
            let result = NoParamReturnArrayPointerCallback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayFloat" => {
            let result = NoParamReturnArrayFloatCallback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayDouble" => {
            let result = NoParamReturnArrayDoubleCallback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayString" => {
            let result = NoParamReturnArrayStringCallback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayAny" => {
            let result = NoParamReturnArrayAnyCallback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayVector2" => {
            let result = NoParamReturnArrayVector2Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayVector3" => {
            let result = NoParamReturnArrayVector3Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayVector4" => {
            let result = NoParamReturnArrayVector4Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnArrayMatrix4x4" => {
            let result = NoParamReturnArrayMatrix4x4Callback();
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "NoParamReturnVector2" => {
            let result = NoParamReturnVector2Callback();
            ReverseReturn(&Str::from(format!("{}", format_vec2(&result))));
        }
        "NoParamReturnVector3" => {
            let result = NoParamReturnVector3Callback();
            ReverseReturn(&Str::from(format!("{}", format_vec3(&result))));
        }
        "NoParamReturnVector4" => {
            let result = NoParamReturnVector4Callback();
            ReverseReturn(&Str::from(format!("{}", format_vec4(&result))));
        }
        "NoParamReturnMatrix4x4" => {
            let result = NoParamReturnMatrix4x4Callback();
            ReverseReturn(&Str::from(format!("{}", format_mat(&result))));
        }
        "Param1" => {
            Param1Callback(999);
        }
        "Param2" => {
            Param2Callback(888, 9.9);
        }
        "Param3" => {
            Param3Callback(777, 8.8, 9.8765);
        }
        "Param4" => {
            Param4Callback(666, 7.7, 8.7659, &Vec4 { x: 100.1, y: 200.2, z: 300.3, w: 400.4 });
        }
        "Param5" => {
            Param5Callback(555, 6.6, 7.6598, &Vec4 { x: -105.1, y: -205.2, z: -305.3, w: -405.4 }, &Arr::new());
        }
        "Param6" => {
            Param6Callback(444, 5.5, 6.5987, &Vec4 { x: 110.1, y: 210.2, z: 310.3, w: 410.4 }, &Arr::from(vec![90000, -100, 20000]), b'A' as i8);
        }
        "Param7" => {
            Param7Callback(333, 4.4, 5.9876, &Vec4 { x: -115.1, y: -215.2, z: -315.3, w: -415.4 }, &Arr::from(vec![800000, 30000, -4000000]), b'B' as i8, &Str::from("red gold"));
        }
        "Param8" => {
            Param8Callback(222, 3.3, 1.2345, &Vec4 { x: 120.1, y: 220.2, z: 320.3, w: 420.4 }, &Arr::from(vec![7000000, 5000000, -600000000]), b'C' as i8, &Str::from("blue ice"), 'Z' as u16);
        }
        "Param9" => {
            Param9Callback(111, 2.2, 5.1234, &Vec4 { x: -125.1, y: -225.2, z: -325.3, w: -425.4 }, &Arr::from(vec![60000000, -700000000, 80000000000]), b'D' as i8, &Str::from("pink metal"), 'Y' as u16, -100);
        }
        "Param10" => {
            Param10Callback(1234, 1.1, 4.5123, &Vec4 { x: 130.1, y: 230.2, z: 330.3, w: 430.4 }, &Arr::from(vec![500000000, 90000000000, 1000000000000]), b'E' as i8, &Str::from("green wood"), 'X' as u16, -200, 0xabeba);
        }
        "ParamRef1" => {
            let mut a = 0i32;
            ParamRef1Callback(&mut a);
            ReverseReturn(&Str::from(format!("{}", a)));
        }
        "ParamRef2" => {
            let mut a = 0i32;
            let mut b = 0f32;
            ParamRef2Callback(&mut a, &mut b);
            ReverseReturn(&Str::from(format!("{}|{}", a, b)));
        }
        "ParamRef3" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            ParamRef3Callback(&mut a, &mut b, &mut c);
            ReverseReturn(&Str::from(format!("{}|{}|{}", a, b, c)));
        }
        "ParamRef4" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            ParamRef4Callback(&mut a, &mut b, &mut c, &mut d);
            ReverseReturn(&Str::from(format!("{}|{}|{}|{}", a, b, c, format_vec4(&d))));
        }
        "ParamRef5" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = Arr::new();
            ParamRef5Callback(&mut a, &mut b, &mut c, &mut d, &mut e);
            ReverseReturn(&Str::from(format!("{}|{}|{}|{}|{}", a, b, c, format_vec4(&d), ArrFormat::format(&e))));
        }
        "ParamRef6" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = Arr::new();
            let mut f = 0i8;
            ParamRef6Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f);
            ReverseReturn(&Str::from(format!("{}|{}|{}|{}|{}|{}", a, b, c, format_vec4(&d), ArrFormat::format(&e), f as u8)));
        }
        "ParamRef7" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = Arr::new();
            let mut f = 0i8;
            let mut g = Str::new();
            ParamRef7Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g);
            ReverseReturn(&Str::from(format!("{}|{}|{}|{}|{}|{}|{}", a, b, c, format_vec4(&d), ArrFormat::format(&e), f, g)));
        }
        "ParamRef8" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = Arr::new();
            let mut f = 0i8;
            let mut g = Str::new();
            let mut h = 0u16;
            ParamRef8Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g, &mut h);
            ReverseReturn(&Str::from(format!("{}|{}|{}|{}|{}|{}|{}|{}", a, b, c, format_vec4(&d), ArrFormat::format(&e), f, g, h)));
        }
        "ParamRef9" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = Arr::new();
            let mut f = 0i8;
            let mut g = Str::new();
            let mut h = 0u16;
            let mut k = 0i16;
            ParamRef9Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g, &mut h, &mut k);
            ReverseReturn(&Str::from(format!("{}|{}|{}|{}|{}|{}|{}|{}|{}", a, b, c, format_vec4(&d), ArrFormat::format(&e), f, g, h, k)));
        }
        "ParamRef10" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = Arr::new();
            let mut f = 0i8;
            let mut g = Str::new();
            let mut h = 0u16;
            let mut k = 0i16;
            let mut l = 0usize;
            ParamRef10Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g, &mut h, &mut k, &mut l);
            ReverseReturn(&Str::from(format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{:p}", a, b, c, format_vec4(&d), ArrFormat::format(&e), f, g, h, k, l as *const ())));
        }
        "ParamRefArrays" => {
            let mut p1 = Arr::from(vec![true]);
            let mut p2 = Arr::from(vec![b'A' as i8]);
            let mut p3 = Arr::from(vec!['A' as u16]);
            let mut p4 = Arr::from(vec![-1i8]);
            let mut p5 = Arr::from(vec![-1i16]);
            let mut p6 = Arr::from(vec![-1i32]);
            let mut p7 = Arr::from(vec![-1i64]);
            let mut p8 = Arr::from(vec![0u8]);
            let mut p9 = Arr::from(vec![0u16]);
            let mut p10 = Arr::from(vec![0u32]);
            let mut p11 = Arr::from(vec![0u64]);
            let mut p12 = Arr::from(vec![0usize]);
            let mut p13 = Arr::from(vec![1.0f32]);
            let mut p14 = Arr::from(vec![1.0f64]);
            let mut p15 = Arr::from(vec![Str::from("Hi")]);
            ParamRefVectorsCallback(&mut p1, &mut p2, &mut p3, &mut p4, &mut p5, &mut p6, &mut p7, &mut p8, &mut p9, &mut p10, &mut p11, &mut p12, &mut p13, &mut p14, &mut p15);
            ReverseReturn(&Str::from(format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                                             ArrFormat::format(&p1),
                                             ArrFormat::format(&p2),
                                             ArrFormat::format(&p3),
                                             ArrFormat::format(&p4),
                                             ArrFormat::format(&p5),
                                             ArrFormat::format(&p6),
                                             ArrFormat::format(&p7),
                                             ArrFormat::format(&p8),
                                             ArrFormat::format(&p9),
                                             ArrFormat::format(&p10),
                                             ArrFormat::format(&p11),
                                             ArrFormat::format(&p12),
                                             ArrFormat::format(&p13),
                                             ArrFormat::format(&p14),
                                             ArrFormat::format(&p15))));
        }
        "ParamAllPrimitives" => {
            let result = ParamAllPrimitivesCallback(true, b'%' as i8, 0x2622, -1, -1000, -1000000, -1000000000000, 200, 50000, 3000000000, 9999999999, 0xfedcbaabcdef, 0.001, 987654.456789);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "ParamEnum" => {
            let p1 = MasterExample::Forth;
            let p2 = Arr::from_slice(&[MasterExample::First, MasterExample::Second, MasterExample::Third]);
            let result = ParamEnumCallback(p1, &p2);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "ParamEnumRef" => {
            let mut p1 = MasterExample::First;
            let mut p2 = Arr::from_slice(&[MasterExample::First, MasterExample::First, MasterExample::Second]);
            let result = ParamEnumRefCallback(&mut p1, &mut p2);
            ReverseReturn(&Str::from(format!("{}|{}|{:?}", result, p1 as i32, p2)));
        }
        "ParamVariant" => {
            let p1 = Var::new(&Any::from("my custom string with enough chars"));
            let p2 = Arr::from(vec![
                Var::new(&Any::Char8(b'X' as i8)),
                Var::new(&Any::Char16(0x2622)),
                Var::new(&Any::Int8(-1)),
                Var::new(&Any::Int16(-1000)),
                Var::new(&Any::Int32(-1000000)),
                Var::new(&Any::Int64(-1000000000000)),
                Var::new(&Any::UInt8(200)),
                Var::new(&Any::UInt16(50000)),
                Var::new(&Any::UInt32(3000000000)),
                Var::new(&Any::UInt64(9999999999)),
                Var::new(&Any::Pointer(0xfedcbaabcdef)),
                Var::new(&Any::Float(0.001)),
                Var::new(&Any::Double(987654.456789)),
            ]);
            ParamVariantCallback(&p1, &p2);
        }
        "ParamVariantRef" => {
            let mut p1 = Var::new(&Any::from("my custom string with enough chars"));
            let mut p2 = Arr::from(vec![
                Var::new(&Any::Char8(b'X' as i8)),
                Var::new(&Any::Char16(0x2622)),
                Var::new(&Any::Int8(-1)),
                Var::new(&Any::Int16(-1000)),
                Var::new(&Any::Int32(-1000000)),
                Var::new(&Any::Int64(-1000000000000)),
                Var::new(&Any::UInt8(200)),
                Var::new(&Any::UInt16(50000)),
                Var::new(&Any::UInt32(3000000000)),
                Var::new(&Any::UInt64(9999999999)),
                Var::new(&Any::Pointer(0xfedcbaabcdef)),
                Var::new(&Any::Float(0.001)),
                Var::new(&Any::Double(987654.456789)),
            ]);
            ParamVariantRefCallback(&mut p1, &mut p2);
            ReverseReturn(&Str::from(format!("{}|{}", format_any(&p1), ArrFormat::format(&p2))));
        }
        "CallFuncVoid" => {
            CallFuncVoidCallback(mock_void);
        }
        "CallFuncBool" => {
            let result = CallFuncBoolCallback(mock_bool);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncChar8" => {
            let result = CallFuncChar8Callback(mock_char8);
            ReverseReturn(&Str::from(format!("{}", result as u8 as char)));
        }
        "CallFuncChar16" => {
            let result = CallFuncChar16Callback(mock_char16);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncInt8" => {
            let result = CallFuncInt8Callback(mock_int8);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncInt16" => {
            let result = CallFuncInt16Callback(mock_int16);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncInt32" => {
            let result = CallFuncInt32Callback(mock_int32);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncInt64" => {
            let result = CallFuncInt64Callback(mock_int64);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncUInt8" => {
            let result = CallFuncUInt8Callback(mock_uint8);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncUInt16" => {
            let result = CallFuncUInt16Callback(mock_uint16);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncUInt32" => {
            let result = CallFuncUInt32Callback(mock_uint32);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncUInt64" => {
            let result = CallFuncUInt64Callback(mock_uint64);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncPtr" => {
            let result = CallFuncPtrCallback(mock_ptr);
            ReverseReturn(&Str::from(format!("{:p}", result as *const())));
        }
        "CallFuncFloat" => {
            let result = CallFuncFloatCallback(mock_float);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncDouble" => {
            let result = CallFuncDoubleCallback(mock_double);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFuncString" => {
            let result = CallFuncStringCallback(mock_string);
            ReverseReturn(&result);
        }
        "CallFuncAny" => {
            let result = CallFuncAnyCallback(mock_any);
            ReverseReturn(&Str::from(format!("{}", format_any(&result))));
        }
        "CallFuncBoolVector" => {
            let result = CallFuncBoolVectorCallback(mock_bool_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncChar8Vector" => {
            let result = CallFuncChar8VectorCallback(mock_char8_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncChar16Vector" => {
            let result = CallFuncChar16VectorCallback(mock_char16_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncInt8Vector" => {
            let result = CallFuncInt8VectorCallback(mock_int8_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncInt16Vector" => {
            let result = CallFuncInt16VectorCallback(mock_int16_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncInt32Vector" => {
            let result = CallFuncInt32VectorCallback(mock_int32_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncInt64Vector" => {
            let result = CallFuncInt64VectorCallback(mock_int64_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncUInt8Vector" => {
            let result = CallFuncUInt8VectorCallback(mock_uint8_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncUInt16Vector" => {
            let result = CallFuncUInt16VectorCallback(mock_uint16_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncUInt32Vector" => {
            let result = CallFuncUInt32VectorCallback(mock_uint32_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncUInt64Vector" => {
            let result = CallFuncUInt64VectorCallback(mock_uint64_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncPtrVector" => {
            let result = CallFuncPtrVectorCallback(mock_ptr_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncFloatVector" => {
            let result = CallFuncFloatVectorCallback(mock_float_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncDoubleVector" => {
            let result = CallFuncDoubleVectorCallback(mock_double_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncStringVector" => {
            let result = CallFuncStringVectorCallback(mock_string_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncAnyVector" => {
            let result = CallFuncAnyVectorCallback(mock_any_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncVec2Vector" => {
            let result = CallFuncVec2VectorCallback(mock_vec2_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncVec3Vector" => {
            let result = CallFuncVec3VectorCallback(mock_vec3_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncVec4Vector" => {
            let result = CallFuncVec4VectorCallback(mock_vec4_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncMat4x4Vector" => {
            let result = CallFuncMat4x4VectorCallback(mock_mat4x4_vector);
            ReverseReturn(&Str::from(format!("{}", ArrFormat::format(&result))));
        }
        "CallFuncVec2" => {
            let result = CallFuncVec2Callback(mock_vec2);
            ReverseReturn(&Str::from(format!("{}",format_vec2(&result))));
        }
        "CallFuncVec3" => {
            let result = CallFuncVec3Callback(mock_vec3);
            ReverseReturn(&Str::from(format!("{}", format_vec3(&result))));
        }
        "CallFuncVec4" => {
            let result = CallFuncVec4Callback(mock_vec4);
            ReverseReturn(&Str::from(format!("{}", format_vec4(&result))));
        }
        "CallFuncMat4x4" => {
            let result = CallFuncMat4x4Callback(mock_mat4x4);
            ReverseReturn(&Str::from(format!("{}", format_mat(&result))));
        }
        "CallFunc1" => {
            let result = CallFunc1Callback(mock_func1);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFunc2" => {
            let result = CallFunc2Callback(mock_func2);
            ReverseReturn(&Str::from(format!("{}", result as u8 as char)));
        }
        "CallFunc3" => {
            CallFunc3Callback(mock_func3);
        }
        "CallFunc4" => {
            let result = CallFunc4Callback(mock_func4);
            ReverseReturn(&Str::from(format!("{}", format_vec4(&result))));
        }
        "CallFunc5" => {
            let result = CallFunc5Callback(mock_func5);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFunc6" => {
            let result = CallFunc6Callback(mock_func6);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFunc7" => {
            let result = CallFunc7Callback(mock_func7);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFunc8" => {
            let result = CallFunc8Callback(mock_func8);
            ReverseReturn(&Str::from(format!("{}", format_mat(&result))));
        }
        "CallFunc9" => {
            CallFunc9Callback(mock_func9);
        }
        "CallFunc10" => {
            let result = CallFunc10Callback(mock_func10);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFunc11" => {
            let result = CallFunc11Callback(mock_func11);
            ReverseReturn(&Str::from(format!("{:p}", result as *const ())));
        }
        "CallFunc12" => {
            let result = CallFunc12Callback(mock_func12);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFunc13" => {
            let result = CallFunc13Callback(mock_func13);
            ReverseReturn(&result);
        }
        "CallFunc14" => {
            let result = CallFunc14Callback(mock_func14);
            ReverseReturn(&Str::from(format!("{}", Arr::format(&result))));
        }
        "CallFunc15" => {
            let result = CallFunc15Callback(mock_func15);
            ReverseReturn(&Str::from(format!("{}", result)));
        }
        "CallFunc16" => {
            let result = CallFunc16Callback(mock_func16);
            ReverseReturn(&Str::from(format!("{:p}", result as *const ())));
        }
        "CallFunc17" => {
            let result = CallFunc17Callback(mock_func17);
            ReverseReturn(&result);
        }
        "CallFunc18" => {
            let result = CallFunc18Callback(mock_func18);
            ReverseReturn(&result);
        }
        "CallFunc19" => {
            let result = CallFunc19Callback(mock_func19);
            ReverseReturn(&result);
        }
        "CallFunc20" => {
            let result = CallFunc20Callback(mock_func20);
            ReverseReturn(&result);
        }
        "CallFunc21" => {
            let result = CallFunc21Callback(mock_func21);
            ReverseReturn(&result);
        }
        "CallFunc22" => {
            let result = CallFunc22Callback(mock_func22);
            ReverseReturn(&result);
        }
        "CallFunc23" => {
            let result = CallFunc23Callback(mock_func23);
            ReverseReturn(&result);
        }
        "CallFunc24" => {
            let result = CallFunc24Callback(mock_func24);
            ReverseReturn(&result);
        }
        "CallFunc25" => {
            let result = CallFunc25Callback(mock_func25);
            ReverseReturn(&result);
        }
        "CallFunc26" => {
            let result = CallFunc26Callback(mock_func26);
            ReverseReturn(&result);
        }
        "CallFunc27" => {
            let result = CallFunc27Callback(mock_func27);
            ReverseReturn(&result);
        }
        "CallFunc28" => {
            let result = CallFunc28Callback(mock_func28);
            ReverseReturn(&result);
        }
        "CallFunc29" => {
            let result = CallFunc29Callback(mock_func29);
            ReverseReturn(&result);
        }
        "CallFunc30" => {
            let result = CallFunc30Callback(mock_func30);
            ReverseReturn(&result);
        }
        "CallFunc31" => {
            let result = CallFunc31Callback(mock_func31);
            ReverseReturn(&result);
        }
        "CallFunc32" => {
            let result = CallFunc32Callback(mock_func32);
            ReverseReturn(&result);
        }
        "CallFunc33" => {
            let result = CallFunc33Callback(mock_func33);
            ReverseReturn(&result);
        }
        "CallFuncEnum" => {
            let result = CallFuncEnumCallback(mock_func_enum);
            ReverseReturn(&result);
        }
        "ClassBasicLifecycle" => {
            let result = basic_lifecycle();
            ReverseReturn(&result);
        }
        "ClassStateManagement" => {
            let result = state_management();
            ReverseReturn(&result);
        }
        "ClassMultipleInstances" => {
            let result = multiple_instances();
            ReverseReturn(&result);
        }
        "ClassCounterWithoutDestructor" => {
            let result = counter_without_destructor();
            ReverseReturn(&result);
        }
        "ClassStaticMethods" => {
            let result = static_methods();
            ReverseReturn(&result);
        }
        "ClassMemoryLeakDetection" => {
            let result = memory_leak_detection();
            ReverseReturn(&result);
        }
        "ClassExceptionHandling" => {
            let result = exception_handling();
            ReverseReturn(&result);
        }
        "ClassOwnershipTransfer" => {
            let result = ownership_transfer();
            ReverseReturn(&result);
        }
        _ => {
            // Unknown test name, do nothing
        }
    }
}