use plugify::{*};
use crate::cross_call_master::*;
type MasterExample = crate::cross_call_master::Example;

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
pub extern "C" fn NoParamReturnString() -> PlgString {
    PlgString::from("Hello World")
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnAny() -> PlgVariant {
    let vec = Vec::from(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    PlgVariant::from(PlgAny::from(vec))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayBool() -> PlgVector<bool> {
    PlgVector::from(vec![true, false])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayChar8() -> PlgVector<i8> {
    PlgVector::from(vec![b'a' as i8, b'b' as i8, b'c' as i8, b'd' as i8])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayChar16() -> PlgVector<u16> {
    PlgVector::from(vec!['a' as u16, 'b' as u16, 'c' as u16, 'd' as u16])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayInt8() -> PlgVector<i8> {
    PlgVector::from(vec![-3, -2, -1, 0, 1])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayInt16() -> PlgVector<i16> {
    PlgVector::from(vec![-4, -3, -2, -1, 0, 1])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayInt32() -> PlgVector<i32> {
    PlgVector::from(vec![-5, -4, -3, -2, -1, 0, 1])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayInt64() -> PlgVector<i64> {
    PlgVector::from(vec![-6, -5, -4, -3, -2, -1, 0, 1])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayUInt8() -> PlgVector<u8> {
    PlgVector::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayUInt16() -> PlgVector<u16> {
    PlgVector::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayUInt32() -> PlgVector<u32> {
    PlgVector::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayUInt64() -> PlgVector<u64> {
    PlgVector::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayPointer() -> PlgVector<usize> {
    PlgVector::from(vec![
        0usize,
        1usize,
        2usize,
        3usize,
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayFloat() -> PlgVector<f32> {
    PlgVector::from(vec![-12.34f32, 0.0f32, 12.34f32])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayDouble() -> PlgVector<f64> {
    PlgVector::from(vec![-12.345, 0.0, 12.345])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayString() -> PlgVector<PlgString> {
    PlgVector::from(vec![
        PlgString::from("1st string"),
        PlgString::from("2nd string"),
        PlgString::from("3rd element string (Should be big enough to avoid small string optimization)"),
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayAny() -> PlgVector<PlgVariant> {
    PlgVector::from(vec![
        PlgAny::from(1.0_f64),
        PlgAny::from(2.0_f32),
        PlgAny::from("3rd element string (Should be big enough to avoid small string optimization)"),
        PlgAny::from(Vec::from(vec![
            String::from("lolek"),
            String::from("and"),
            String::from("bolek"),
        ])),
        PlgAny::from(1_i32),
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayVector2() -> PlgVector<Vector2> {
    PlgVector::from(vec![
        Vector2 { x: 1.1, y: 2.2 },
        Vector2 { x: -3.3, y: 4.4 },
        Vector2 { x: 5.5, y: -6.6 },
        Vector2 { x: 7.7, y: 8.8 },
        Vector2 { x: 0.0, y: 0.0 },
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayVector3() -> PlgVector<Vector3> {
    PlgVector::from(vec![
        Vector3 { x: 1.1, y: 2.2, z: 3.3 },
        Vector3 { x: -4.4, y: 5.5, z: -6.6 },
        Vector3 { x: 7.7, y: 8.8, z: 9.9 },
        Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        Vector3 { x: 10.1, y: -11.2, z: 12.3 },
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayVector4() -> PlgVector<Vector4> {
    PlgVector::from(vec![
        Vector4 { x: 1.1, y: 2.2, z: 3.3, w: 4.4 },
        Vector4 { x: -5.5, y: 6.6, z: -7.7, w: 8.8 },
        Vector4 { x: 9.9, y: 0.0, z: -1.1, w: 2.2 },
        Vector4 { x: 3.3, y: 4.4, z: 5.5, w: 6.6 },
        Vector4 { x: -7.7, y: -8.8, z: 9.9, w: -10.1 },
    ])
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnArrayMatrix4x4() -> PlgVector<Matrix4x4> {
    PlgVector::from(vec![
        Matrix4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0], // Identity matrix
            ]
        },
        Matrix4x4 {
            m: [
                [2.0, 3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0, 9.0],
                [10.0, 11.0, 12.0, 13.0],
                [14.0, 15.0, 16.0, 17.0], // Example random matrix
            ]
        },
        Matrix4x4 {
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
pub extern "C" fn NoParamReturnVector2() -> Vector2 {
    Vector2 { x: 1.0, y: 2.0 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnVector3() -> Vector3 {
    Vector3 { x: 1.0, y: 2.0, z: 3.0 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnVector4() -> Vector4 {
    Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn NoParamReturnMatrix4x4() -> Matrix4x4 {
    Matrix4x4 {
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
pub extern "C" fn Param4(a: i32, b: f32, c: f64, d: &Vector4) {
    let _buffer = format!("{}{:.2}{:.2}{:.1}{:.1}", a, b, c, d.x, d.w);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param5(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>) {
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
pub extern "C" fn Param6(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8) {
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
pub extern "C" fn Param7(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8, g: &PlgString) {
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
        g.as_str()
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param8(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8, g: &PlgString, h: u16) {
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
        g.as_str(),
        h
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param9(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8, g: &PlgString, h: u16, k: i16) {
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
        g.as_str(),
        h,
        k
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn Param10(a: i32, b: f32, c: f64, d: &Vector4, e: &PlgVector<i64>, f: i8, g: &PlgString, h: u16, k: i16, l: usize) {
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
        g.as_str(),
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
    *c = std::f64::consts::PI;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef4(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4) {
    *a = 100;
    *b = -5.55;
    *c = 1.618;
    *d = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef5(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>) {
    *a = 500;
    *b = -10.5;
    *c = std::f64::consts::E;
    *d = Vector4 { x: -1.0, y: -2.0, z: -3.0, w: -4.0 };
    e.set(&[-6, -5, -4, -3, -2, -1, 0, 1]);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef6(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8) {
    *a = 750;
    *b = 20.0;
    *c = 1.23456;
    *d = Vector4 { x: 10.0, y: 20.0, z: 30.0, w: 40.0 };
    e.set(&[-6, -5, -4]);
    *f = b'Z' as i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef7(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8, g: &mut PlgString) {
    *a = -1000;
    *b = 3.0;
    *c = -1.0;
    *d = Vector4 { x: 100.0, y: 200.0, z: 300.0, w: 400.0 };
    e.set(&[-6, -5, -4, -3]);
    *f = b'Y' as i8;
    g.set("Hello, World!");
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef8(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8, g: &mut PlgString, h: &mut u16) {
    *a = 999;
    *b = -7.5;
    *c = 0.123456;
    *d = Vector4 { x: -100.0, y: -200.0, z: -300.0, w: -400.0 };
    e.set(&[-6, -5, -4, -3, -2, -1]);
    *f = b'X' as i8;
    g.set("Goodbye, World!");
    *h = 'A' as u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef9(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8, g: &mut PlgString, h: &mut u16, k: &mut i16) {
    *a = -1234;
    *b = 123.45;
    *c = -678.9;
    *d = Vector4 { x: 987.65, y: 432.1, z: 123.456, w: 789.123 };
    e.set(&[-6, -5, -4, -3, -2, -1, 0, 1, 5, 9]);
    *f = b'W' as i8;
    g.set("Testing, 1 2 3");
    *h = 'B' as u16;
    *k = 42;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamRef10(a: &mut i32, b: &mut f32, c: &mut f64, d: &mut Vector4, e: &mut PlgVector<i64>, f: &mut i8, g: &mut PlgString, h: &mut u16, k: &mut i16, l: &mut usize) {
    *a = 987;
    *b = -0.123;
    *c = 456.789;
    *d = Vector4 { x: -123.456, y: 0.987, z: 654.321, w: -789.123 };
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
    p1: &mut PlgVector<bool>,
    p2: &mut PlgVector<i8>,
    p3: &mut PlgVector<u16>,
    p4: &mut PlgVector<i8>,
    p5: &mut PlgVector<i16>,
    p6: &mut PlgVector<i32>,
    p7: &mut PlgVector<i64>,
    p8: &mut PlgVector<u8>,
    p9: &mut PlgVector<u16>,
    p10: &mut PlgVector<u32>,
    p11: &mut PlgVector<u64>,
    p12: &mut PlgVector<usize>,
    p13: &mut PlgVector<f32>,
    p14: &mut PlgVector<f64>,
    p15: &mut PlgVector<PlgString>,
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
        PlgString::from("1"),
        PlgString::from("12"),
        PlgString::from("123"),
        PlgString::from("1234"),
        PlgString::from("12345"),
        PlgString::from("123456"),
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
pub extern "C" fn ParamEnum(p1: Example, p2: &PlgVector<Example>) -> i32 {
    let sum: i32 = p2.iter().map(|e| *e as i32).sum();
    (p1 as i32) + sum
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamEnumRef(p1: &mut Example, p2: &mut PlgVector<Example>) -> i32 {
    *p1 = Example::Forth;

    let new_values = vec![Example::First, Example::Second, Example::Third];
    p2.set(&new_values);

    let sum: i32 = p2.iter().map(|e| *e as i32).sum();
    (*p1 as i32) + sum
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamVariant(p1: &PlgVariant, p2: &PlgVector<PlgVariant>) {
    let _buffer = format!("{:?}|{}", p1.get(), p2.len());
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ParamVariantRef(p1: &mut PlgVariant, p2: &mut PlgVector<PlgVariant>) {
    p1.set(&PlgAny::Char8(b'Z' as i8));

    let variants = vec![
        PlgVariant::new(&PlgAny::Bool(false)),
        PlgVariant::new(&PlgAny::Double(6.28)),
        PlgVariant::new(&PlgAny::ArrayDouble(vec![1.0, 2.0, 3.0])),
        PlgVariant::new(&PlgAny::Pointer(0)),
        PlgVariant::new(&PlgAny::Int32(123456789)),
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
type FuncString = extern "C" fn() -> PlgString;
type FuncAny = extern "C" fn() -> PlgVariant;

type FuncBoolVector = extern "C" fn() -> PlgVector<bool>;
type FuncChar8Vector = extern "C" fn() -> PlgVector<i8>;
type FuncChar16Vector = extern "C" fn() -> PlgVector<u16>;
type FuncInt8Vector = extern "C" fn() -> PlgVector<i8>;
type FuncInt16Vector = extern "C" fn() -> PlgVector<i16>;
type FuncInt32Vector = extern "C" fn() -> PlgVector<i32>;
type FuncInt64Vector = extern "C" fn() -> PlgVector<i64>;
type FuncUInt8Vector = extern "C" fn() -> PlgVector<u8>;
type FuncUInt16Vector = extern "C" fn() -> PlgVector<u16>;
type FuncUInt32Vector = extern "C" fn() -> PlgVector<u32>;
type FuncUInt64Vector = extern "C" fn() -> PlgVector<u64>;
type FuncPtrVector = extern "C" fn() -> PlgVector<usize>;
type FuncFloatVector = extern "C" fn() -> PlgVector<f32>;
type FuncDoubleVector = extern "C" fn() -> PlgVector<f64>;
type FuncStringVector = extern "C" fn() -> PlgVector<PlgString>;
type FuncAnyVector = extern "C" fn() -> PlgVector<PlgVariant>;
type FuncVec2Vector = extern "C" fn() -> PlgVector<Vector2>;
type FuncVec3Vector = extern "C" fn() -> PlgVector<Vector3>;
type FuncVec4Vector = extern "C" fn() -> PlgVector<Vector4>;
type FuncMat4x4Vector = extern "C" fn() -> PlgVector<Matrix4x4>;

type FuncVec2 = extern "C" fn() -> Vector2;
type FuncVec3 = extern "C" fn() -> Vector3;
type FuncVec4 = extern "C" fn() -> Vector4;
type FuncMat4x4 = extern "C" fn() -> Matrix4x4;

type Func1 = extern "C" fn(&Vector3) -> i32;
type Func2 = extern "C" fn(f32, i64) -> i8;
type Func3 = extern "C" fn(usize, &Vector4, &PlgString);
type Func4 = extern "C" fn(bool, i32, u16, &Matrix4x4) -> Vector4;
type Func5 = extern "C" fn(i8, &Vector2, usize, f64, &PlgVector<u64>) -> bool;
type Func6 = extern "C" fn(&PlgString, f32, &PlgVector<f32>, i16, &PlgVector<u8>, usize) -> i64;
type Func7 = extern "C" fn(&PlgVector<i8>, u16, u16, &PlgVector<u32>, &Vector4, bool, u64) -> f64;
type Func8 = extern "C" fn(&Vector3, &PlgVector<u32>, i16, bool, &Vector4, &PlgVector<u16>, u16, i32) -> Matrix4x4;
type Func9 = extern "C" fn(f32, &Vector2, &PlgVector<i8>, u64, bool, &PlgString, &Vector4, i16, usize);
type Func10 = extern "C" fn(&Vector4, &Matrix4x4, &PlgVector<u32>, u64, &PlgVector<i8>, i32, bool, &Vector2, i64, f64) -> u32;
type Func12 = extern "C" fn(usize, &PlgVector<f64>, u32, f64, bool, i32, i8, u64, f32, &PlgVector<usize>, i64, i8) -> bool;
type Func13 = extern "C" fn(i64, &PlgVector<i8>, u16, f32, &PlgVector<bool>, &Vector4, &PlgString, i32, &Vector3, usize, &Vector2, &PlgVector<u8>, i16) -> PlgString;
type Func14 = extern "C" fn(&PlgVector<i8>, &PlgVector<u32>, &Matrix4x4, bool, u16, i32, &PlgVector<f32>, u16, &PlgVector<u8>, i8, &Vector3, &Vector4, f64, usize) -> PlgVector<PlgString>;
type Func15 = extern "C" fn(&PlgVector<i16>, &Matrix4x4, &Vector4, usize, u64, &PlgVector<u32>, bool, f32, &PlgVector<u16>, u8, i32, &Vector2, u16, f64, &PlgVector<u8>) -> i16;
type Func17 = extern "C" fn(&mut i32);
type Func18 = extern "C" fn(&mut i8, &mut i16) -> Vector2;
type Func19 = extern "C" fn(&mut u32, &mut Vector3, &mut PlgVector<u32>);
type Func20 = extern "C" fn(&mut u16, &mut Vector4, &mut PlgVector<u64>, &mut i8) -> i32;
type Func21 = extern "C" fn(&mut Matrix4x4, &mut PlgVector<i32>, &mut Vector2, &mut bool, &mut f64) -> f32;
type Func22 = extern "C" fn(&mut usize, &mut u32, &mut PlgVector<f64>, &mut i16, &mut PlgString, &mut Vector4) -> u64;
type Func23 = extern "C" fn(&mut u64, &mut Vector2, &mut PlgVector<i16>, &mut u16, &mut f32, &mut i8, &mut PlgVector<u8>);
type Func24 = extern "C" fn(&mut PlgVector<i8>, &mut i64, &mut PlgVector<u8>, &mut Vector4, &mut u64, &mut PlgVector<usize>, &mut f64, &mut PlgVector<usize>) -> Matrix4x4;
type Func25 = extern "C" fn(&mut i32, &mut PlgVector<usize>, &mut bool, &mut u8, &mut PlgString, &mut Vector3, &mut i64, &mut Vector4, &mut u16) -> f64;
type Func26 = extern "C" fn(&mut u16, &mut Vector2, &mut Matrix4x4, &mut PlgVector<f32>, &mut i16, &mut u64, &mut u32, &mut PlgVector<u16>, &mut usize, &mut bool) -> i8;
type Func27 = extern "C" fn(&mut f32, &mut Vector3, &mut usize, &mut Vector2, &mut PlgVector<i16>, &mut Matrix4x4, &mut bool, &mut Vector4, &mut i8, &mut i32, &mut PlgVector<u8>) -> u8;
type Func28 = extern "C" fn(&mut usize, &mut u16, &mut PlgVector<u32>, &mut Matrix4x4, &mut f32, &mut Vector4, &mut PlgString, &mut PlgVector<u64>, &mut i64, &mut bool, &mut Vector3, &mut PlgVector<f32>) -> PlgString;
type Func29 = extern "C" fn(&mut Vector4, &mut i32, &mut PlgVector<i8>, &mut f64, &mut bool, &mut i8, &mut PlgVector<u16>, &mut f32, &mut PlgString, &mut Matrix4x4, &mut u64, &mut Vector3, &mut PlgVector<i64>) -> PlgVector<PlgString>;
type Func30 = extern "C" fn(&mut usize, &mut Vector4, &mut i64, &mut PlgVector<u32>, &mut bool, &mut PlgString, &mut Vector3, &mut PlgVector<u8>, &mut f32, &mut Vector2, &mut Matrix4x4, &mut i8, &mut PlgVector<f32>, &mut f64) -> i32;
type Func31 = extern "C" fn(&mut i8, &mut u32, &mut PlgVector<u64>, &mut Vector4, &mut PlgString, &mut bool, &mut i64, &mut Vector2, &mut i8, &mut u16, &mut PlgVector<i16>, &mut Matrix4x4, &mut Vector3, &mut f32, &mut PlgVector<f64>) -> Vector3;
type Func32 = extern "C" fn(&mut i32, &mut u16, &mut PlgVector<i8>, &mut Vector4, &mut usize, &mut PlgVector<u32>, &mut Matrix4x4, &mut u64, &mut PlgString, &mut i64, &mut Vector2, &mut PlgVector<i8>, &mut bool, &mut Vector3, &mut u8, &mut PlgVector<u16>) -> f64;
type Func33 = extern "C" fn(&mut PlgVariant);
type FuncEnum = extern "C" fn(Example, &mut PlgVector<Example>) -> PlgVector<Example>;

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
pub extern "C" fn CallFuncString(func: FuncString) -> PlgString {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncAny(func: FuncAny) -> PlgVariant {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncBoolVector(func: FuncBoolVector) -> PlgVector<bool> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncChar8Vector(func: FuncChar8Vector) -> PlgVector<i8> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncChar16Vector(func: FuncChar16Vector) -> PlgVector<u16> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt8Vector(func: FuncInt8Vector) -> PlgVector<i8> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt16Vector(func: FuncInt16Vector) -> PlgVector<i16> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt32Vector(func: FuncInt32Vector) -> PlgVector<i32> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncInt64Vector(func: FuncInt64Vector) -> PlgVector<i64> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt8Vector(func: FuncUInt8Vector) -> PlgVector<u8> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt16Vector(func: FuncUInt16Vector) -> PlgVector<u16> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt32Vector(func: FuncUInt32Vector) -> PlgVector<u32> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncUInt64Vector(func: FuncUInt64Vector) -> PlgVector<u64> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncPtrVector(func: FuncPtrVector) -> PlgVector<usize> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncFloatVector(func: FuncFloatVector) -> PlgVector<f32> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncDoubleVector(func: FuncDoubleVector) -> PlgVector<f64> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncStringVector(func: FuncStringVector) -> PlgVector<PlgString> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncAnyVector(func: FuncAnyVector) -> PlgVector<PlgVariant> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec2Vector(func: FuncVec2Vector) -> PlgVector<Vector2> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec3Vector(func: FuncVec3Vector) -> PlgVector<Vector3> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec4Vector(func: FuncVec4Vector) -> PlgVector<Vector4> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncMat4x4Vector(func: FuncMat4x4Vector) -> PlgVector<Matrix4x4> {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec2(func: FuncVec2) -> Vector2 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec3(func: FuncVec3) -> Vector3 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncVec4(func: FuncVec4) -> Vector4 {
    func()
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncMat4x4(func: FuncMat4x4) -> Matrix4x4 {
    func()
}

// ============================================================================
// CallFunc Functions - Complex (with parameters)
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc1(func: Func1) -> i32 {
    let vec = Vector3 { x: 4.5, y: 5.6, z: 6.7 };
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
    let vec4 = Vector4 { x: 7.8, y: 8.9, z: 9.1, w: 10.2 };
    let str = PlgString::from("RandomString");
    func(ptr, &vec4, &str);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc4(func: Func4) -> Vector4 {
    let b = false;
    let u32 = 42i32;
    let ch16 = 'B' as u16;
    let mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    func(b, u32, ch16, &mat)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc5(func: Func5) -> bool {
    let i8 = 10i8;
    let vec2 = Vector2 { x: 3.4, y: 5.6 };
    let ptr = 67890usize;
    let d = 1.618f64;
    let vec64 = PlgVector::from(vec![4u64, 5, 6]);
    func(i8, &vec2, ptr, d, &vec64)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc6(func: Func6) -> i64 {
    let str = PlgString::from("AnotherString");
    let f = 4.56f32;
    let vec_f = PlgVector::from(vec![4.0f32, 5.0, 6.0]);
    let i16 = 30i16;
    let vec_u8 = PlgVector::from(vec![3u8, 4, 5]);
    let ptr = 24680usize;
    func(&str, f, &vec_f, i16, &vec_u8, ptr)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc7(func: Func7) -> f64 {
    let vec_c = PlgVector::from(vec![b'X' as i8, b'Y' as i8, b'Z' as i8]);
    let u16 = 20u16;
    let ch16 = 'C' as u16;
    let vec_u32 = PlgVector::from(vec![4u32, 5, 6]);
    let vec4 = Vector4 { x: 4.5, y: 5.6, z: 6.7, w: 7.8 };
    let b = false;
    let u64 = 200u64;
    func(&vec_c, u16, ch16, &vec_u32, &vec4, b, u64)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc8(func: Func8) -> Matrix4x4 {
    let vec3 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let vec_u32 = PlgVector::from(vec![4u32, 5, 6]);
    let i16 = 30i16;
    let b = false;
    let vec4 = Vector4 { x: 4.5, y: 5.6, z: 6.7, w: 7.8 };
    let vec_c16 = PlgVector::from(vec!['D' as u16, 'E' as u16]);
    let ch16 = 'B' as u16;
    let i32 = 50i32;
    func(&vec3, &vec_u32, i16, b, &vec4, &vec_c16, ch16, i32)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc9(func: Func9) {
    let f = 2.71f32;
    let vec2 = Vector2 { x: 3.4, y: 5.6 };
    let vec_i8 = PlgVector::from(vec![4i8, 5, 6]);
    let u64 = 250u64;
    let b = false;
    let str = PlgString::from("Random");
    let vec4 = Vector4 { x: 4.5, y: 5.6, z: 6.7, w: 7.8 };
    let i16 = 30i16;
    let ptr = 13579usize;
    func(f, &vec2, &vec_i8, u64, b, &str, &vec4, i16, ptr);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc10(func: Func10) -> u32 {
    let vec4 = Vector4 { x: 5.6, y: 7.8, z: 8.9, w: 9.0 };
    let mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let vec_u32 = PlgVector::from(vec![4u32, 5, 6]);
    let u64 = 150u64;
    let vec_c = PlgVector::from(vec![b'X' as i8, b'Y' as i8, b'Z' as i8]);
    let i32 = 60i32;
    let b = false;
    let vec2 = Vector2 { x: 3.4, y: 5.6 };
    let i64 = 75i64;
    let d = 2.71f64;
    func(&vec4, &mat, &vec_u32, u64, &vec_c, i32, b, &vec2, i64, d)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc12(func: Func12) -> bool {
    let ptr = 98765usize;
    let vec_d = PlgVector::from(vec![4.0f64, 5.0, 6.0]);
    let u32 = 30u32;
    let d = 1.41f64;
    let b = false;
    let i32 = 25i32;
    let i8 = 10i8;
    let u64 = 300u64;
    let f = 2.72f32;
    let vec_ptr = PlgVector::from(vec![2usize, 3usize, 4usize]);
    let i64 = 200i64;
    let ch = b'B' as i8;
    func(ptr, &vec_d, u32, d, b, i32, i8, u64, f, &vec_ptr, i64, ch)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc13(func: Func13) -> PlgString {
    let i64 = 75i64;
    let vec_c = PlgVector::from(vec![b'D' as i8, b'E' as i8, b'F' as i8]);
    let u16 = 20u16;
    let f = 2.71f32;
    let vec_b = PlgVector::from(vec![false, true, false]);
    let vec4 = Vector4 { x: 5.6, y: 7.8, z: 9.0, w: 10.1 };
    let str = PlgString::from("RandomString");
    let i32 = 30i32;
    let vec3 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let ptr = 13579usize;
    let vec2 = Vector2 { x: 4.5, y: 6.7 };
    let vec_u8 = PlgVector::from(vec![2u8, 3, 4]);
    let i16 = 20i16;
    func(i64, &vec_c, u16, f, &vec_b, &vec4, &str, i32, &vec3, ptr, &vec2, &vec_u8, i16)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc14(func: Func14) -> PlgVector<PlgString> {
    let vec_c = PlgVector::from(vec![b'D' as i8, b'E' as i8, b'F' as i8]);
    let vec_u32 = PlgVector::from(vec![4u32, 5, 6]);
    let mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let b = false;
    let ch16 = 'B' as u16;
    let i32 = 25i32;
    let vec_f = PlgVector::from(vec![4.0f32, 5.0, 6.0]);
    let u16 = 30u16;
    let vec_u8 = PlgVector::from(vec![3u8, 4, 5]);
    let i8 = 10i8;
    let vec3 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let vec4 = Vector4 { x: 5.6, y: 7.8, z: 9.0, w: 10.1 };
    let d = 2.72f64;
    let ptr = 54321usize;
    func(&vec_c, &vec_u32, &mat, b, ch16, i32, &vec_f, u16, &vec_u8, i8, &vec3, &vec4, d, ptr)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc15(func: Func15) -> i16 {
    let vec_i16 = PlgVector::from(vec![4i16, 5, 6]);
    let mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let vec4 = Vector4 { x: 7.8, y: 8.9, z: 9.0, w: 10.1 };
    let ptr = 12345usize;
    let u64 = 200u64;
    let vec_u32 = PlgVector::from(vec![5u32, 6, 7]);
    let b = false;
    let f = 3.14f32;
    let vec_c16 = PlgVector::from(vec!['D' as u16, 'E' as u16]);
    let u8 = 6u8;
    let i32 = 25i32;
    let vec2 = Vector2 { x: 5.6, y: 7.8 };
    let u16 = 40u16;
    let d = 2.71f64;
    let vec_u8 = PlgVector::from(vec![1u8, 3, 5]);
    func(&vec_i16, &mat, &vec4, ptr, u64, &vec_u32, b, f, &vec_c16, u8, i32, &vec2, u16, d, &vec_u8)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc17(func: Func17) -> PlgString {
    let mut i32 = 42i32;
    func(&mut i32);
    PlgString::from(format!("{}", i32))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc18(func: Func18) -> PlgString {
    let mut i8 = 9i8;
    let mut i16 = 25i16;
    let ret = func(&mut i8, &mut i16);
    PlgString::from(format!("{}|{}|{}", format!("{},{}", ret.x, ret.y), i8, i16))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc19(func: Func19) -> PlgString {
    let mut u32 = 75u32;
    let mut vec3 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut vec_u32 = PlgVector::from(vec![4u32, 5, 6]);
    func(&mut u32, &mut vec3, &mut vec_u32);
    PlgString::from(format!("{}|{}|{}", u32, format!("{},{},{}", vec3.x, vec3.y, vec3.z), vec_u32.len()))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc20(func: Func20) -> PlgString {
    let mut ch16 = 'Z' as u16;
    let mut vec4 = Vector4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut vec_u64 = PlgVector::from(vec![4u64, 5, 6]);
    let mut ch = b'X' as i8;
    let ret = func(&mut ch16, &mut vec4, &mut vec_u64, &mut ch);
    PlgString::from(format!("{}|{}|{}|{}|{}", ret, ch16, format!("{},{},{},{}", vec4.x, vec4.y, vec4.z, vec4.w), vec_u64.len(), ch as u8))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc21(func: Func21) -> PlgString {
    let mut mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let mut vec_i32 = PlgVector::from(vec![4i32, 5, 6]);
    let mut vec2 = Vector2 { x: 3.0, y: 4.0 };
    let mut b = false;
    let mut d = 6.28f64;
    let ret = func(&mut mat, &mut vec_i32, &mut vec2, &mut b, &mut d);
    PlgString::from(format!("{}|{}|{}|{}|{}", ret, "mat", vec_i32.len(), format!("{},{}", vec2.x, vec2.y), if b { "true" } else { "false" }))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc22(func: Func22) -> PlgString {
    let mut ptr = 1usize;
    let mut u32 = 20u32;
    let mut vec_d = PlgVector::from(vec![4.0f64, 5.0, 6.0]);
    let mut i16 = 15i16;
    let mut str = PlgString::from("Updated Test");
    let mut vec4 = Vector4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let ret = func(&mut ptr, &mut u32, &mut vec_d, &mut i16, &mut str, &mut vec4);
    PlgString::from(format!("{}|{}|{}|{}|{}|{}", ret, ptr, u32, vec_d.len(), i16, str.as_str()))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc23(func: Func23) -> PlgString {
    let mut u64 = 200u64;
    let mut vec2 = Vector2 { x: 3.0, y: 4.0 };
    let mut vec_i16 = PlgVector::from(vec![4i16, 5, 6]);
    let mut ch16 = 'Y' as u16;
    let mut f = 2.34f32;
    let mut i8 = 10i8;
    let mut vec_u8 = PlgVector::from(vec![3u8, 4, 5]);
    func(&mut u64, &mut vec2, &mut vec_i16, &mut ch16, &mut f, &mut i8, &mut vec_u8);
    PlgString::from(format!("{}|{}|{}|{}|{}|{}", u64, format!("{},{}", vec2.x, vec2.y), vec_i16.len(), ch16, f, i8))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc24(func: Func24) -> PlgString {
    let mut vec_c = PlgVector::from(vec![b'D' as i8, b'E' as i8, b'F' as i8]);
    let mut i64 = 100i64;
    let mut vec_u8 = PlgVector::from(vec![3u8, 4, 5]);
    let mut vec4 = Vector4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut u64 = 200u64;
    let mut vec_ptr = PlgVector::from(vec![3usize, 4usize, 5usize]);
    let mut d = 6.28f64;
    let mut vec_v2 = PlgVector::from(vec![4usize, 5usize, 6usize, 7usize]);
    let _ret = func(&mut vec_c, &mut i64, &mut vec_u8, &mut vec4, &mut u64, &mut vec_ptr, &mut d, &mut vec_v2);
    PlgString::from(format!("{}|{}|{}|{}|{}|{}|{}", "mat", vec_c.len(), i64, vec_u8.len(), format!("{},{},{},{}", vec4.x, vec4.y, vec4.z, vec4.w), u64, vec_ptr.len()))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc25(func: Func25) -> PlgString {
    let mut i32 = 50i32;
    let mut vec_ptr = PlgVector::from(vec![3usize, 4usize, 5usize]);
    let mut b = false;
    let mut u8 = 10u8;
    let mut str = PlgString::from("Updated Test String");
    let mut vec3 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut i64 = 100i64;
    let mut vec4 = Vector4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut u16 = 20u16;
    let ret = func(&mut i32, &mut vec_ptr, &mut b, &mut u8, &mut str, &mut vec3, &mut i64, &mut vec4, &mut u16);
    PlgString::from(format!("{}|{}|{}|{}|{}", ret, i32, vec_ptr.len(), if b { "true" } else { "false" }, u8))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc26(func: Func26) -> PlgString {
    let mut ch16 = 'B' as u16;
    let mut vec2 = Vector2 { x: 3.0, y: 4.0 };
    let mut mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let mut vec_f = PlgVector::from(vec![4.0f32, 5.0, 6.0]);
    let mut i16 = 20i16;
    let mut u64 = 200u64;
    let mut u32 = 20u32;
    let mut vec_u16 = PlgVector::from(vec![3u16, 4, 5]);
    let mut ptr = 0xDEADBEAFDEADBEAFusize;
    let mut b = false;
    let ret = func(&mut ch16, &mut vec2, &mut mat, &mut vec_f, &mut i16, &mut u64, &mut u32, &mut vec_u16, &mut ptr, &mut b);
    PlgString::from(format!("{}|{}|{}|{}|{}|{}", ret as u8, ch16, format!("{},{}", vec2.x, vec2.y), "mat", vec_f.len(), u64))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc27(func: Func27) -> PlgString {
    let mut f = 2.56f32;
    let mut vec3 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut ptr = 0usize;
    let mut vec2 = Vector2 { x: 3.0, y: 4.0 };
    let mut vec_i16 = PlgVector::from(vec![4i16, 5, 6]);
    let mut mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let mut b = false;
    let mut vec4 = Vector4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut i8 = 10i8;
    let mut i32 = 40i32;
    let mut vec_u8 = PlgVector::from(vec![3u8, 4, 5]);
    let ret = func(&mut f, &mut vec3, &mut ptr, &mut vec2, &mut vec_i16, &mut mat, &mut b, &mut vec4, &mut i8, &mut i32, &mut vec_u8);
    PlgString::from(format!("{}|{}|{}|{}|{}", ret, f, format!("{},{},{}", vec3.x, vec3.y, vec3.z), ptr, format!("{},{}", vec2.x, vec2.y)))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc28(func: Func28) -> PlgString {
    let mut ptr = 1usize;
    let mut u16 = 20u16;
    let mut vec_u32 = PlgVector::from(vec![4u32, 5, 6]);
    let mut mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let mut f = 2.71f32;
    let mut vec4 = Vector4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let mut str = PlgString::from("New example string");
    let mut vec_u64 = PlgVector::from(vec![400u64, 500, 600]);
    let mut i64 = 987654321i64;
    let mut b = false;
    let mut vec3 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut vec_f = PlgVector::from(vec![4.0f32, 5.0, 6.0]);
    let ret = func(&mut ptr, &mut u16, &mut vec_u32, &mut mat, &mut f, &mut vec4, &mut str, &mut vec_u64, &mut i64, &mut b, &mut vec3, &mut vec_f);
    PlgString::from(format!("{}|{}|{}|{}|{}", ret.as_str(), ptr, u16, vec_u32.len(), "mat"))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc29(func: Func29) -> PlgString {
    let mut vec4 = Vector4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
    let mut i32 = 99i32;
    let mut vec_i8 = PlgVector::from(vec![4i8, 5, 6]);
    let mut d = 2.71f64;
    let mut b = false;
    let mut i8 = 10i8;
    let mut vec_u16 = PlgVector::from(vec![4u16, 5, 6]);
    let mut f = 3.21f32;
    let mut str = PlgString::from("Yet another example string");
    let mut mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let mut u64 = 200u64;
    let mut vec3 = Vector3 { x: 5.0, y: 6.0, z: 7.0 };
    let mut vec_i64 = PlgVector::from(vec![2000i64, 3000, 4000]);
    let ret = func(&mut vec4, &mut i32, &mut vec_i8, &mut d, &mut b, &mut i8, &mut vec_u16, &mut f, &mut str, &mut mat, &mut u64, &mut vec3, &mut vec_i64);
    PlgString::from(format!("{}|{}|{}|{}|{}", ret.len(), format!("{},{},{},{}", vec4.x, vec4.y, vec4.z, vec4.w), i32, vec_i8.len(), d))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc30(func: Func30) -> PlgString {
    let mut ptr = 1usize;
    let mut vec4 = Vector4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
    let mut i64 = 987654321i64;
    let mut vec_u32 = PlgVector::from(vec![4u32, 5, 6]);
    let mut b = false;
    let mut str = PlgString::from("Updated String for Func30");
    let mut vec3 = Vector3 { x: 5.0, y: 6.0, z: 7.0 };
    let mut vec_u8 = PlgVector::from(vec![1u8, 2, 3]);
    let mut f = 5.67f32;
    let mut vec2 = Vector2 { x: 3.0, y: 4.0 };
    let mut mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let mut i8 = 10i8;
    let mut vec_f = PlgVector::from(vec![4.0f32, 5.0, 6.0]);
    let mut d = 8.90f64;
    let ret = func(&mut ptr, &mut vec4, &mut i64, &mut vec_u32, &mut b, &mut str, &mut vec3, &mut vec_u8, &mut f, &mut vec2, &mut mat, &mut i8, &mut vec_f, &mut d);
    PlgString::from(format!("{}|{}|{}|{}|{}", ret, ptr, format!("{},{},{},{}", vec4.x, vec4.y, vec4.z, vec4.w), i64, vec_u32.len()))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc31(func: Func31) -> PlgString {
    let mut ch = b'B' as i8;
    let mut u32 = 200u32;
    let mut vec_u64 = PlgVector::from(vec![4u64, 5, 6]);
    let mut vec4 = Vector4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
    let mut str = PlgString::from("Updated String for Func31");
    let mut b = true;
    let mut i64 = 987654321i64;
    let mut vec2 = Vector2 { x: 3.0, y: 4.0 };
    let mut i8 = 10i8;
    let mut u16 = 20u16;
    let mut vec_i16 = PlgVector::from(vec![4i16, 5, 6]);
    let mut mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let mut vec3 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut f = 5.67f32;
    let mut vec_d = PlgVector::from(vec![4.0f64, 5.0, 6.0]);
    let ret = func(&mut ch, &mut u32, &mut vec_u64, &mut vec4, &mut str, &mut b, &mut i64, &mut vec2, &mut i8, &mut u16, &mut vec_i16, &mut mat, &mut vec3, &mut f, &mut vec_d);
    PlgString::from(format!("{}|{}|{}|{}|{}", format!("{},{},{}", ret.x, ret.y, ret.z), ch as u8, u32, vec_u64.len(), format!("{},{},{},{}", vec4.x, vec4.y, vec4.z, vec4.w)))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc32(func: Func32) -> PlgString {
    let mut i32 = 30i32;
    let mut u16 = 20u16;
    let mut vec_i8 = PlgVector::from(vec![4i8, 5, 6]);
    let mut vec4 = Vector4 { x: 2.0, y: 3.0, z: 4.0, w: 5.0 };
    let mut ptr = 1usize;
    let mut vec_u32 = PlgVector::from(vec![4u32, 5, 6]);
    let mut mat = Matrix4x4 { m: [[0.0; 4]; 4] };
    let mut u64 = 200u64;
    let mut str = PlgString::from("Updated String for Func32");
    let mut i64 = 987654321i64;
    let mut vec2 = Vector2 { x: 3.0, y: 4.0 };
    let mut vec_i8_2 = PlgVector::from(vec![7i8, 8, 9]);
    let mut b = false;
    let mut vec3 = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let mut u8 = 128u8;
    let mut vec_c16 = PlgVector::from(vec!['D' as u16, 'E' as u16, 'F' as u16]);
    let _ret = func(&mut i32, &mut u16, &mut vec_i8, &mut vec4, &mut ptr, &mut vec_u32, &mut mat, &mut u64, &mut str, &mut i64, &mut vec2, &mut vec_i8_2, &mut b, &mut vec3, &mut u8, &mut vec_c16);
    PlgString::from(format!("{}|{}|{}|{}|{}", i32, u16, vec_i8.len(), format!("{},{},{},{}", vec4.x, vec4.y, vec4.z, vec4.w), ptr))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFunc33(func: Func33) -> PlgString {
    let mut variant = PlgVariant::new(&PlgAny::Int32(30));
    func(&mut variant);
    PlgString::from(format!("{:?}", variant.get()))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn CallFuncEnum(func: FuncEnum) -> PlgString {
    let p1 = Example::Forth;
    let mut p2 = PlgVector::from_slice(&[Example::Forth, Example::Forth, Example::Forth]);
    let ret = func(p1, &mut p2);
    PlgString::from(format!("{}|{}", ret.len(), p2.len()))
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
extern "C" fn mock_string() -> PlgString { PlgString::from("Test string") }
extern "C" fn mock_any() -> PlgVariant { PlgVariant::from(PlgAny::Char16('A' as u16)) }

extern "C" fn mock_bool_vector() -> PlgVector<bool> { PlgVector::from(vec![true, false]) }
extern "C" fn mock_char8_vector() -> PlgVector<i8> { PlgVector::from(vec![b'A' as i8, b'B' as i8]) }
extern "C" fn mock_char16_vector() -> PlgVector<u16> { PlgVector::from(vec!['A' as u16, 'B' as u16]) }
extern "C" fn mock_int8_vector() -> PlgVector<i8> { PlgVector::from(vec![10, 20]) }
extern "C" fn mock_int16_vector() -> PlgVector<i16> { PlgVector::from(vec![100, 200]) }
extern "C" fn mock_int32_vector() -> PlgVector<i32> { PlgVector::from(vec![1000, 2000]) }
extern "C" fn mock_int64_vector() -> PlgVector<i64> { PlgVector::from(vec![10000, 20000]) }
extern "C" fn mock_uint8_vector() -> PlgVector<u8> { PlgVector::from(vec![20, 30]) }
extern "C" fn mock_uint16_vector() -> PlgVector<u16> { PlgVector::from(vec![200, 300]) }
extern "C" fn mock_uint32_vector() -> PlgVector<u32> { PlgVector::from(vec![2000, 3000]) }
extern "C" fn mock_uint64_vector() -> PlgVector<u64> { PlgVector::from(vec![20000, 30000]) }
extern "C" fn mock_ptr_vector() -> PlgVector<usize> { PlgVector::from(vec![0, 1]) }
extern "C" fn mock_float_vector() -> PlgVector<f32> { PlgVector::from(vec![1.1, 2.2]) }
extern "C" fn mock_double_vector() -> PlgVector<f64> { PlgVector::from(vec![3.3, 4.4]) }
extern "C" fn mock_string_vector() -> PlgVector<PlgString> {
    PlgVector::from(vec![PlgString::from("Hello"), PlgString::from("World")])
}
extern "C" fn mock_any_vector() -> PlgVector<PlgVariant> {
    PlgVector::from(vec![
        PlgAny::from("Hello"),
        PlgAny::from(3.14f32),
        PlgAny::from(6.28f64),
        PlgAny::from(1i32),
        PlgAny::from(0xDEADBEAF_usize),
    ])
}

extern "C" fn mock_vec2_vector() -> PlgVector<Vector2> {
    PlgVector::from(vec![
        Vector2 { x: 0.5, y: -1.2 },
        Vector2 { x: 3.4, y: 7.8 },
        Vector2 { x: -6.7, y: 2.3 },
        Vector2 { x: 8.9, y: -4.5 },
        Vector2 { x: 0.0, y: 0.0 },
    ])
}

extern "C" fn mock_vec3_vector() -> PlgVector<Vector3> {
    PlgVector::from(vec![
        Vector3 { x: 2.1, y: 3.2, z: 4.3 },
        Vector3 { x: -5.4, y: 6.5, z: -7.6 },
        Vector3 { x: 8.7, y: 9.8, z: 0.1 },
        Vector3 { x: 1.2, y: -3.3, z: 4.4 },
        Vector3 { x: -5.5, y: 6.6, z: -7.7 },
    ])
}

extern "C" fn mock_vec4_vector() -> PlgVector<Vector4> {
    PlgVector::from(vec![
        Vector4 { x: 0.1, y: 1.2, z: 2.3, w: 3.4 },
        Vector4 { x: -4.5, y: 5.6, z: 6.7, w: -7.8 },
        Vector4 { x: 8.9, y: -9.0, z: 10.1, w: -11.2 },
        Vector4 { x: 12.3, y: 13.4, z: 14.5, w: 15.6 },
        Vector4 { x: -16.7, y: 17.8, z: 18.9, w: -19.0 },
    ])
}

extern "C" fn mock_mat4x4_vector() -> PlgVector<Matrix4x4> {
    PlgVector::from(vec![
        Matrix4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        },
        Matrix4x4 {
            m: [
                [0.5, 1.0, 1.5, 2.0],
                [2.5, 3.0, 3.5, 4.0],
                [4.5, 5.0, 5.5, 6.0],
                [6.5, 7.0, 7.5, 8.0],
            ]
        },
        Matrix4x4 {
            m: [
                [-1.0, -2.0, -3.0, -4.0],
                [-5.0, -6.0, -7.0, -8.0],
                [-9.0, -10.0, -11.0, -12.0],
                [-13.0, -14.0, -15.0, -16.0],
            ]
        },
        Matrix4x4 {
            m: [
                [1.1, 2.2, 3.3, 4.4],
                [5.5, 6.6, 7.7, 8.8],
                [9.9, 10.0, 11.1, 12.2],
                [13.3, 14.4, 15.5, 16.6],
            ]
        },
    ])
}

extern "C" fn mock_vec2() -> Vector2 { Vector2 { x: 1.0, y: 2.0 } }
extern "C" fn mock_vec3() -> Vector3 { Vector3 { x: 1.0, y: 2.0, z: 3.0 } }
extern "C" fn mock_vec4() -> Vector4 { Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 } }
extern "C" fn mock_mat4x4() -> Matrix4x4 {
    Matrix4x4 {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]
    }
}

// Mock implementations for 1+ parameter functions
extern "C" fn mock_func1(v: &Vector3) -> i32 {
    let _buffer = format!("{}{}{}", v.x, v.y, v.z);
    (v.x + v.y + v.z) as i32
}

extern "C" fn mock_func2(a: f32, b: i64) -> i8 {
    let _buffer = format!("{}{}", a, b);
    b'&' as i8
}

extern "C" fn mock_func3(p: usize, v: &Vector4, s: &PlgString) {
    let _buffer = format!("{}{}{}{}{}{}", p, v.x, v.y, v.z, v.w, s.as_str());
}

extern "C" fn mock_func4(flag: bool, u: i32, c: u16, _m: &Matrix4x4) -> Vector4 {
    let _buffer = format!("{}{}{}", flag, u, c);
    Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 }
}

extern "C" fn mock_func5(i: i8, v: &Vector2, p: usize, d: f64, vec: &PlgVector<u64>) -> bool {
    let _buffer = format!("{}{}{}{}{}{}", i, v.x, v.y, p, d, vec.len());
    true
}

extern "C" fn mock_func6(s: &PlgString, f: f32, vec: &PlgVector<f32>, i: i16, u_vec: &PlgVector<u8>, p: usize) -> i64 {
    let _buffer = format!("{}{}{}{}{}{}", s.as_str(), f, vec.len(), i, u_vec.len(), p);
    (f + i as f32) as i64
}

extern "C" fn mock_func7(vec: &PlgVector<i8>, u: u16, c: u16, u_vec: &PlgVector<u32>, v: &Vector4, flag: bool, l: u64) -> f64 {
    let _buffer = format!("{}{}{}{}{}{}{}{}{}{}", vec.len(), u, c, u_vec.len(), v.x, v.y, v.z, v.w, flag, l);
    3.14
}

extern "C" fn mock_func8(_v: &Vector3, _u_vec: &PlgVector<u32>, _i: i16, _flag: bool, _v4: &Vector4, _c_vec: &PlgVector<u16>, _c: u16, _a: i32) -> Matrix4x4 {
    Matrix4x4 { m: [[0.0; 4]; 4] }
}

extern "C" fn mock_func9(_f: f32, _v: &Vector2, _i_vec: &PlgVector<i8>, _l: u64, _flag: bool, _s: &PlgString, _v4: &Vector4, _i: i16, _p: usize) {
}

extern "C" fn mock_func10(_v4: &Vector4, _m: &Matrix4x4, _u_vec: &PlgVector<u32>, _l: u64, _c_vec: &PlgVector<i8>, _a: i32, _flag: bool, _v: &Vector2, _i: i64, _d: f64) -> u32 {
    42
}

extern "C" fn mock_func11(b_vec: &PlgVector<bool>, c: u16, u: u8, d: f64, v3: &Vector3, i_vec: &PlgVector<i8>, i: i64, u16: u16, f: f32, v: &Vector2, u32: u32) -> usize {
    let _buffer = format!("{}{}{}{}{}{}{}{}{}{}{}", b_vec.len(), c, u, d, v3.x, i_vec.len(), i, u16, f, v.x, u32);
    0
}

extern "C" fn mock_func12(p: usize, d_vec: &PlgVector<f64>, u: u32, d: f64, flag: bool, a: i32, i: i8, l: u64, f: f32, p_vec: &PlgVector<usize>, i64: i64, c: i8) -> bool {
    let _buffer = format!("{}{}{}{}{}{}{}{}{}{}{}{}", p, d_vec.len(), u, d, flag, a, i, l, f, p_vec.len(), i64, c);
    false
}

extern "C" fn mock_func13(_i64: i64, _c_vec: &PlgVector<i8>, _u16: u16, _f: f32, _b_vec: &PlgVector<bool>, _v4: &Vector4, _s: &PlgString, _a: i32, _v3: &Vector3, _p: usize, _v2: &Vector2, _u8_vec: &PlgVector<u8>, _i16: i16) -> PlgString {
    PlgString::from("Dummy String")
}

extern "C" fn mock_func14(_c_vec: &PlgVector<i8>, _u_vec: &PlgVector<u32>, _m: &Matrix4x4, _flag: bool, _c: u16, _a: i32, _f_vec: &PlgVector<f32>, _u16: u16, _u8_vec: &PlgVector<u8>, _i8: i8, _v3: &Vector3, _v4: &Vector4, _d: f64, _p: usize) -> PlgVector<PlgString> {
    PlgVector::from(vec![PlgString::from("String1"), PlgString::from("String2")])
}

extern "C" fn mock_func15(_i_vec: &PlgVector<i16>, _m: &Matrix4x4, _v4: &Vector4, _p: usize, _l: u64, _u_vec: &PlgVector<u32>, _flag: bool, _f: f32, _c_vec: &PlgVector<u16>, _u: u8, _a: i32, _v2: &Vector2, _u16: u16, _d: f64, _u8_vec: &PlgVector<u8>) -> i16 {
    257
}

extern "C" fn mock_func16(b_vec: &PlgVector<bool>, i16: i16, i_vec: &PlgVector<i8>, v4: &Vector4, m: &Matrix4x4, v2: &Vector2, u_vec: &PlgVector<u64>, c_vec: &PlgVector<i8>, s: &PlgString, i64: i64, u32_vec: &PlgVector<u32>, v3: &Vector3, f: f32, d: f64, i8: i8, u16: u16) -> usize {
    let _buffer = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", i16, b_vec.len(), i_vec.len(), v4.x, v4.y, v4.z, v4.w, m.m[3][3], v2.x, u_vec.len(), c_vec.len(), s.as_str(), i64, u32_vec.len(), v3.x, f, d, i8, u16);
    0
}

extern "C" fn mock_func17(r: &mut i32) {
    *r += 10;
}

extern "C" fn mock_func18(i8: &mut i8, i16: &mut i16) -> Vector2 {
    *i8 = 5;
    *i16 = 10;
    Vector2 { x: *i8 as f32, y: *i16 as f32 }
}

extern "C" fn mock_func19(u32: &mut u32, v3: &mut Vector3, u_vec: &mut PlgVector<u32>) {
    *u32 = 42;
    *v3 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    u_vec.set(&[1, 2, 3]);
}

extern "C" fn mock_func20(c: &mut u16, v4: &mut Vector4, u_vec: &mut PlgVector<u64>, ch: &mut i8) -> i32 {
    *c = 't' as u16;
    *v4 = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    u_vec.set(&[100, 200]);
    *ch = b'F' as i8;
    0
}

extern "C" fn mock_func21(m: &mut Matrix4x4, i_vec: &mut PlgVector<i32>, v2: &mut Vector2, flag: &mut bool, d: &mut f64) -> f32 {
    *flag = true;
    *d = 3.14;
    *v2 = Vector2 { x: 1.0, y: 2.0 };
    *m = Matrix4x4 {
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

extern "C" fn mock_func22(p: &mut usize, u32: &mut u32, d_vec: &mut PlgVector<f64>, i16: &mut i16, s: &mut PlgString, v4: &mut Vector4) -> u64 {
    *p = 0;
    *u32 = 99;
    *i16 = 123;
    s.set("Hello");
    *v4 = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    d_vec.set(&[1.1, 2.2, 3.3]);
    0
}

extern "C" fn mock_func23(u64: &mut u64, v2: &mut Vector2, i_vec: &mut PlgVector<i16>, c: &mut u16, f: &mut f32, i8: &mut i8, u8_vec: &mut PlgVector<u8>) {
    *u64 = 50;
    *f = 1.5;
    *i8 = -1;
    *v2 = Vector2 { x: 3.0, y: 4.0 };
    u8_vec.set(&[1, 2, 3]);
    *c = 0x2164; // Roman numeral V character
    i_vec.set(&[1, 2, 3, 4]);
}

extern "C" fn mock_func24(c_vec: &mut PlgVector<i8>, i64: &mut i64, u8_vec: &mut PlgVector<u8>, v4: &mut Vector4, u64: &mut u64, p_vec: &mut PlgVector<usize>, d: &mut f64, v_vec: &mut PlgVector<usize>) -> Matrix4x4 {
    *i64 = 64;
    *d = 2.71;
    *v4 = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    c_vec.set(&[b'a' as i8, b'b' as i8, b'c' as i8]);
    u8_vec.set(&[5, 6, 7]);
    p_vec.set(&[0]);
    v_vec.set(&[1, 1, 2, 2]);
    *u64 = 0xffffffff;
    Matrix4x4 { m: [[0.0; 4]; 4] }
}

extern "C" fn mock_func25(i32: &mut i32, p_vec: &mut PlgVector<usize>, flag: &mut bool, u8: &mut u8, s: &mut PlgString, v3: &mut Vector3, i64: &mut i64, v4: &mut Vector4, u16: &mut u16) -> f64 {
    *flag = false;
    *i32 = 100;
    *u8 = 250;
    *v3 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    *v4 = Vector4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 };
    s.set("MockFunc25");
    p_vec.set(&[0]);
    *i64 = 1337;
    *u16 = 64222;
    0.0
}

extern "C" fn mock_func26(c: &mut u16, v2: &mut Vector2, m: &mut Matrix4x4, f_vec: &mut PlgVector<f32>, i16: &mut i16, u64: &mut u64, u32: &mut u32, u16_vec: &mut PlgVector<u16>, p: &mut usize, flag: &mut bool) -> i8 {
    *c = 'Z' as u16;
    *flag = true;
    *v2 = Vector2 { x: 2.0, y: 3.0 };
    *m = Matrix4x4 {
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

extern "C" fn mock_func27(f: &mut f32, v3: &mut Vector3, p: &mut usize, v2: &mut Vector2, i16_vec: &mut PlgVector<i16>, m: &mut Matrix4x4, flag: &mut bool, v4: &mut Vector4, i8: &mut i8, i32: &mut i32, u8_vec: &mut PlgVector<u8>) -> u8 {
    *f = 1.0;
    *v3 = Vector3 { x: -1.0, y: -2.0, z: -3.0 };
    *p = 0xDEADBEAFDEADBEAF;
    *v2 = Vector2 { x: -111.0, y: 111.0 };
    i16_vec.set(&[1, 2, 3, 4]);
    *m = Matrix4x4 {
        m: [
            [1.0, 0.5, 0.3, 0.7],
            [0.8, 1.2, 0.6, 0.9],
            [1.5, 1.1, 0.4, 0.2],
            [0.3, 0.9, 0.7, 1.0],
        ]
    };
    *flag = true;
    *v4 = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    *i8 = 111;
    *i32 = 30;
    u8_vec.set(&[0, 0, 0, 0, 0, 0, 1, 0]);
    0
}

extern "C" fn mock_func28(ptr: &mut usize, u16: &mut u16, u32_vec: &mut PlgVector<u32>, m: &mut Matrix4x4, f: &mut f32, v4: &mut Vector4, str: &mut PlgString, u64_vec: &mut PlgVector<u64>, i64: &mut i64, b: &mut bool, vec3: &mut Vector3, f_vec: &mut PlgVector<f32>) -> PlgString {
    *ptr = 0;
    *u16 = 65500;
    u32_vec.set(&[1, 2, 3, 4, 5, 7]);
    *m = Matrix4x4 {
        m: [
            [1.4, 0.7, 0.2, 0.5],
            [0.3, 1.1, 0.6, 0.8],
            [0.9, 0.4, 1.3, 0.1],
            [0.6, 0.2, 0.7, 1.0],
        ]
    };
    *f = 5.5;
    *v4 = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    u64_vec.set(&[1, 2]);
    *i64 = 834748377834;
    *b = true;
    *vec3 = Vector3 { x: 10.0, y: 20.0, z: 30.0 };
    str.set("MockFunc28");
    f_vec.set(&[1.0, -1000.0, 2000.0]);
    PlgString::from("MockFunc28")
}

extern "C" fn mock_func29(v4: &mut Vector4, i32: &mut i32, i_vec: &mut PlgVector<i8>, d: &mut f64, flag: &mut bool, i8: &mut i8, u16_vec: &mut PlgVector<u16>, f: &mut f32, s: &mut PlgString, m: &mut Matrix4x4, u64: &mut u64, v3: &mut Vector3, i64_vec: &mut PlgVector<i64>) -> PlgVector<PlgString> {
    *i32 = 30;
    *flag = true;
    *v4 = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    *d = 3.14;
    *i8 = 8;
    u16_vec.set(&[100, 200]);
    *f = 1.5;
    s.set("MockFunc29");
    *m = Matrix4x4 {
        m: [
            [0.4, 1.0, 0.6, 0.3],
            [1.2, 0.8, 0.5, 0.9],
            [0.7, 0.3, 1.4, 0.6],
            [0.1, 0.9, 0.8, 1.3],
        ]
    };
    *u64 = 64;
    *v3 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    i64_vec.set(&[1, 2, 3]);
    i_vec.set(&[127, 126, 125]);
    PlgVector::from(vec![PlgString::from("Example"), PlgString::from("MockFunc29")])
}

extern "C" fn mock_func30(p: &mut usize, v4: &mut Vector4, i64: &mut i64, u_vec: &mut PlgVector<u32>, flag: &mut bool, s: &mut PlgString, v3: &mut Vector3, u8_vec: &mut PlgVector<u8>, f: &mut f32, v2: &mut Vector2, m: &mut Matrix4x4, i8: &mut i8, vec: &mut PlgVector<f32>, d: &mut f64) -> i32 {
    *flag = false;
    *f = 1.1;
    *i64 = 1000;
    *v2 = Vector2 { x: 3.0, y: 4.0 };
    *v4 = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    s.set("MockFunc30");
    *p = 0;
    u_vec.set(&[100, 200]);
    *m = Matrix4x4 {
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
    *v3 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    u8_vec.set(&[255, 0, 255, 200, 100, 200]);
    42
}

extern "C" fn mock_func31(c: &mut i8, u32: &mut u32, u_vec: &mut PlgVector<u64>, v4: &mut Vector4, s: &mut PlgString, flag: &mut bool, i64: &mut i64, v2: &mut Vector2, i8: &mut i8, u16: &mut u16, i_vec: &mut PlgVector<i16>, m: &mut Matrix4x4, v3: &mut Vector3, f: &mut f32, v4_vec: &mut PlgVector<f64>) -> Vector3 {
    *u32 = 12345;
    *flag = true;
    *v3 = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    *c = b'C' as i8;
    *v4 = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    s.set("MockFunc31");
    *i64 = 123456789;
    *v2 = Vector2 { x: 5.0, y: 6.0 };
    *i8 = 7;
    *u16 = 255;
    i_vec.set(&[1, 2]);
    *m = Matrix4x4 {
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
    Vector3 { x: 1.0, y: 1.5, z: 3.0 }
}

extern "C" fn mock_func32(i32: &mut i32, u16: &mut u16, i_vec: &mut PlgVector<i8>, v4: &mut Vector4, p: &mut usize, u_vec: &mut PlgVector<u32>, m: &mut Matrix4x4, u64: &mut u64, s: &mut PlgString, i64: &mut i64, v2: &mut Vector2, u8_vec: &mut PlgVector<i8>, flag: &mut bool, v3: &mut Vector3, u8: &mut u8, c_vec: &mut PlgVector<u16>) -> f64 {
    *i32 = 42;
    *u16 = 255;
    *flag = false;
    *v2 = Vector2 { x: 2.5, y: 3.5 };
    u8_vec.set(&[1, 2, 3, 4, 5, 9]);
    *v4 = Vector4 { x: 4.0, y: 5.0, z: 6.0, w: 7.0 };
    s.set("MockFunc32");
    *p = 0;
    *m = Matrix4x4 {
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
    *v3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    *u8 = 8;
    c_vec.set(&['a' as u16, 'b' as u16, 'c' as u16]);
    i_vec.set(&[0, 1]);
    1.0
}

extern "C" fn mock_func33(variant: &mut PlgVariant) {
    variant.set(&PlgAny::from("MockFunc33"));
}

extern "C" fn mock_func_enum(p1: MasterExample, p2: &mut PlgVector<MasterExample>) -> PlgVector<MasterExample> {
    p2.set(&[MasterExample::First, MasterExample::Second, MasterExample::Third]);
    PlgVector::from_slice(&[p1, MasterExample::Forth])
}

// ============================================================================
// ReverseCall Function
// ============================================================================

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn ReverseCall(test: &PlgString) {
    let test_name = test.as_str();

    match test_name {
        "NoParamReturnVoid" => {
            NoParamReturnVoidCallback();
        }
        "NoParamReturnBool" => {
            let result = NoParamReturnBoolCallback();
            let result_str = if result { "true" } else { "false" };
            ReverseReturn(&PlgString::from(result_str));
        }
        "NoParamReturnChar8" => {
            let result = NoParamReturnChar8Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnChar16" => {
            let result = NoParamReturnChar16Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnInt8" => {
            let result = NoParamReturnInt8Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnInt16" => {
            let result = NoParamReturnInt16Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnInt32" => {
            let result = NoParamReturnInt32Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnInt64" => {
            let result = NoParamReturnInt64Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnUInt8" => {
            let result = NoParamReturnUInt8Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnUInt16" => {
            let result = NoParamReturnUInt16Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnUInt32" => {
            let result = NoParamReturnUInt32Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnUInt64" => {
            let result = NoParamReturnUInt64Callback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnPointer" => {
            let result = NoParamReturnPointerCallback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnFloat" => {
            let result = NoParamReturnFloatCallback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnDouble" => {
            let result = NoParamReturnDoubleCallback();
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "NoParamReturnFunction" => {
            let result = NoParamReturnFunctionCallback();
            let res;
            unsafe { res = result(); }
            ReverseReturn(&PlgString::from(res.to_string()));
        }
        "NoParamReturnString" => {
            let result = NoParamReturnStringCallback();
            ReverseReturn(&result);
        }
        "NoParamReturnAny" => {
            let result = NoParamReturnAnyCallback();
            ReverseReturn(&PlgString::from(format!("{:?}", result.get())));
        }
        "NoParamReturnArrayBool" => {
            let result = NoParamReturnArrayBoolCallback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayChar8" => {
            let result = NoParamReturnArrayChar8Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayChar16" => {
            let result = NoParamReturnArrayChar16Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayInt8" => {
            let result = NoParamReturnArrayInt8Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayInt16" => {
            let result = NoParamReturnArrayInt16Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayInt32" => {
            let result = NoParamReturnArrayInt32Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayInt64" => {
            let result = NoParamReturnArrayInt64Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayUInt8" => {
            let result = NoParamReturnArrayUInt8Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayUInt16" => {
            let result = NoParamReturnArrayUInt16Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayUInt32" => {
            let result = NoParamReturnArrayUInt32Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayUInt64" => {
            let result = NoParamReturnArrayUInt64Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayPointer" => {
            let result = NoParamReturnArrayPointerCallback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayFloat" => {
            let result = NoParamReturnArrayFloatCallback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayDouble" => {
            let result = NoParamReturnArrayDoubleCallback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayString" => {
            let result = NoParamReturnArrayStringCallback();
            let strings: Vec<String> = result.iter().map(|s| s.as_str().to_string()).collect();
            ReverseReturn(&PlgString::from(format!("{:?}", strings)));
        }
        "NoParamReturnArrayAny" => {
            let result = NoParamReturnArrayAnyCallback();
            let anys: Vec<String> = result.iter().map(|a| format!("{:?}", a.get())).collect();
            ReverseReturn(&PlgString::from(format!("{:?}", anys)));
        }
        "NoParamReturnArrayVector2" => {
            let result = NoParamReturnArrayVector2Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayVector3" => {
            let result = NoParamReturnArrayVector3Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayVector4" => {
            let result = NoParamReturnArrayVector4Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnArrayMatrix4x4" => {
            let result = NoParamReturnArrayMatrix4x4Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnVector2" => {
            let result = NoParamReturnVector2Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnVector3" => {
            let result = NoParamReturnVector3Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnVector4" => {
            let result = NoParamReturnVector4Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "NoParamReturnMatrix4x4" => {
            let result = NoParamReturnMatrix4x4Callback();
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
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
            Param4Callback(666, 7.7, 8.7659, &Vector4 { x: 100.1, y: 200.2, z: 300.3, w: 400.4 });
        }
        "Param5" => {
            Param5Callback(555, 6.6, 7.6598, &Vector4 { x: -105.1, y: -205.2, z: -305.3, w: -405.4 }, &PlgVector::new());
        }
        "Param6" => {
            Param6Callback(444, 5.5, 6.5987, &Vector4 { x: 110.1, y: 210.2, z: 310.3, w: 410.4 }, &PlgVector::from(vec![90000, -100, 20000]), b'A' as i8);
        }
        "Param7" => {
            Param7Callback(333, 4.4, 5.9876, &Vector4 { x: -115.1, y: -215.2, z: -315.3, w: -415.4 }, &PlgVector::from(vec![800000, 30000, -4000000]), b'B' as i8, &PlgString::from("red gold"));
        }
        "Param8" => {
            Param8Callback(222, 3.3, 1.2345, &Vector4 { x: 120.1, y: 220.2, z: 320.3, w: 420.4 }, &PlgVector::from(vec![7000000, 5000000, -600000000]), b'C' as i8, &PlgString::from("blue ice"), 'Z' as u16);
        }
        "Param9" => {
            Param9Callback(111, 2.2, 5.1234, &Vector4 { x: -125.1, y: -225.2, z: -325.3, w: -425.4 }, &PlgVector::from(vec![60000000, -700000000, 80000000000]), b'D' as i8, &PlgString::from("pink metal"), 'Y' as u16, -100);
        }
        "Param10" => {
            Param10Callback(1234, 1.1, 4.5123, &Vector4 { x: 130.1, y: 230.2, z: 330.3, w: 430.4 }, &PlgVector::from(vec![500000000, 90000000000, 1000000000000]), b'E' as i8, &PlgString::from("green wood"), 'X' as u16, -200, 0xabeba);
        }
        "ParamRef1" => {
            let mut a = 0i32;
            ParamRef1Callback(&mut a);
            ReverseReturn(&PlgString::from(format!("{}", a)));
        }
        "ParamRef2" => {
            let mut a = 0i32;
            let mut b = 0f32;
            ParamRef2Callback(&mut a, &mut b);
            ReverseReturn(&PlgString::from(format!("{}|{}", a, b)));
        }
        "ParamRef3" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            ParamRef3Callback(&mut a, &mut b, &mut c);
            ReverseReturn(&PlgString::from(format!("{}|{}|{}", a, b, c)));
        }
        "ParamRef4" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            ParamRef4Callback(&mut a, &mut b, &mut c, &mut d);
            ReverseReturn(&PlgString::from(format!("{}|{}|{}|{:?}", a, b, c, d)));
        }
        "ParamRef5" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = PlgVector::new();
            ParamRef5Callback(&mut a, &mut b, &mut c, &mut d, &mut e);
            ReverseReturn(&PlgString::from(format!("{}|{}|{}|{:?}|{:?}", a, b, c, d, e)));
        }
        "ParamRef6" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = PlgVector::new();
            let mut f = 0i8;
            ParamRef6Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f);
            ReverseReturn(&PlgString::from(format!("{}|{}|{}|{:?}|{:?}|{}", a, b, c, d, e, f as u8)));
        }
        "ParamRef7" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = PlgVector::new();
            let mut f = 0i8;
            let mut g = PlgString::new();
            ParamRef7Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g);
            ReverseReturn(&PlgString::from(format!("{}|{}|{}|{:?}|{:?}|{}|{}", a, b, c, d, e, f as u8, g.as_str())));
        }
        "ParamRef8" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = PlgVector::new();
            let mut f = 0i8;
            let mut g = PlgString::new();
            let mut h = 0u16;
            ParamRef8Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g, &mut h);
            ReverseReturn(&PlgString::from(format!("{}|{}|{}|{:?}|{:?}|{}|{}|{}", a, b, c, d, e, f as u8, g.as_str(), h)));
        }
        "ParamRef9" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = PlgVector::new();
            let mut f = 0i8;
            let mut g = PlgString::new();
            let mut h = 0u16;
            let mut k = 0i16;
            ParamRef9Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g, &mut h, &mut k);
            ReverseReturn(&PlgString::from(format!("{}|{}|{}|{:?}|{:?}|{}|{}|{}|{}", a, b, c, d, e, f as u8, g.as_str(), h, k)));
        }
        "ParamRef10" => {
            let mut a = 0i32;
            let mut b = 0f32;
            let mut c = 0f64;
            let mut d = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
            let mut e = PlgVector::new();
            let mut f = 0i8;
            let mut g = PlgString::new();
            let mut h = 0u16;
            let mut k = 0i16;
            let mut l = 0usize;
            ParamRef10Callback(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f, &mut g, &mut h, &mut k, &mut l);
            ReverseReturn(&PlgString::from(format!("{}|{}|{}|{:?}|{:?}|{}|{}|{}|{}|{}", a, b, c, d, e, f as u8, g.as_str(), h, k, l)));
        }
        "ParamRefArrays" => {
            let mut p1 = PlgVector::from(vec![true]);
            let mut p2 = PlgVector::from(vec![b'A' as i8]);
            let mut p3 = PlgVector::from(vec!['A' as u16]);
            let mut p4 = PlgVector::from(vec![-1i8]);
            let mut p5 = PlgVector::from(vec![-1i16]);
            let mut p6 = PlgVector::from(vec![-1i32]);
            let mut p7 = PlgVector::from(vec![-1i64]);
            let mut p8 = PlgVector::from(vec![0u8]);
            let mut p9 = PlgVector::from(vec![0u16]);
            let mut p10 = PlgVector::from(vec![0u32]);
            let mut p11 = PlgVector::from(vec![0u64]);
            let mut p12 = PlgVector::from(vec![0usize]);
            let mut p13 = PlgVector::from(vec![1.0f32]);
            let mut p14 = PlgVector::from(vec![1.0f64]);
            let mut p15 = PlgVector::from(vec![PlgString::from("Hi")]);
            ParamRefVectorsCallback(&mut p1, &mut p2, &mut p3, &mut p4, &mut p5, &mut p6, &mut p7, &mut p8, &mut p9, &mut p10, &mut p11, &mut p12, &mut p13, &mut p14, &mut p15);
            let p15_strings: Vec<String> = p15.iter().map(|s| s.as_str().to_string()).collect();
            ReverseReturn(&PlgString::from(format!("{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
                                                   p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15_strings)));
        }
        "ParamAllPrimitives" => {
            let result = ParamAllPrimitivesCallback(true, b'%' as i8, 0x2622, -1, -1000, -1000000, -1000000000000, 200, 50000, 3000000000, 9999999999, 0xfedcbaabcdef, 0.001, 987654.456789);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "ParamEnum" => {
            let p1 = MasterExample::Forth;
            let p2 = PlgVector::from_slice(&[MasterExample::First, MasterExample::Second, MasterExample::Third]);
            let result = ParamEnumCallback(p1, &p2);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "ParamEnumRef" => {
            let mut p1 = MasterExample::First;
            let mut p2 = PlgVector::from_slice(&[MasterExample::First, MasterExample::First, MasterExample::Second]);
            let result = ParamEnumRefCallback(&mut p1, &mut p2);
            ReverseReturn(&PlgString::from(format!("{}|{}|{:?}", result, p1 as i32, p2)));
        }
        "ParamVariant" => {
            let p1 = PlgVariant::new(&PlgAny::from("my custom string with enough chars"));
            let p2 = PlgVector::from(vec![
                PlgVariant::new(&PlgAny::Char8(b'X' as i8)),
                PlgVariant::new(&PlgAny::Char16(0x2622)),
                PlgVariant::new(&PlgAny::Int8(-1)),
                PlgVariant::new(&PlgAny::Int16(-1000)),
                PlgVariant::new(&PlgAny::Int32(-1000000)),
                PlgVariant::new(&PlgAny::Int64(-1000000000000)),
                PlgVariant::new(&PlgAny::UInt8(200)),
                PlgVariant::new(&PlgAny::UInt16(50000)),
                PlgVariant::new(&PlgAny::UInt32(3000000000)),
                PlgVariant::new(&PlgAny::UInt64(9999999999)),
                PlgVariant::new(&PlgAny::Pointer(0xfedcbaabcdef)),
                PlgVariant::new(&PlgAny::Float(0.001)),
                PlgVariant::new(&PlgAny::Double(987654.456789)),
            ]);
            ParamVariantCallback(&p1, &p2);
        }
        "ParamVariantRef" => {
            let mut p1 = PlgVariant::new(&PlgAny::from("my custom string with enough chars"));
            let mut p2 = PlgVector::from(vec![
                PlgVariant::new(&PlgAny::Char8(b'X' as i8)),
                PlgVariant::new(&PlgAny::Char16(0x2622)),
                PlgVariant::new(&PlgAny::Int8(-1)),
                PlgVariant::new(&PlgAny::Int16(-1000)),
                PlgVariant::new(&PlgAny::Int32(-1000000)),
                PlgVariant::new(&PlgAny::Int64(-1000000000000)),
                PlgVariant::new(&PlgAny::UInt8(200)),
                PlgVariant::new(&PlgAny::UInt16(50000)),
                PlgVariant::new(&PlgAny::UInt32(3000000000)),
                PlgVariant::new(&PlgAny::UInt64(9999999999)),
                PlgVariant::new(&PlgAny::Pointer(0xfedcbaabcdef)),
                PlgVariant::new(&PlgAny::Float(0.001)),
                PlgVariant::new(&PlgAny::Double(987654.456789)),
            ]);
            ParamVariantRefCallback(&mut p1, &mut p2);
            let p2_str: Vec<String> = p2.iter().map(|v| format!("{:?}", v.get())).collect();
            ReverseReturn(&PlgString::from(format!("{:?}|{:?}", p1.get(), p2_str)));
        }
        "CallFuncVoid" => {
            CallFuncVoidCallback(mock_void);
        }
        "CallFuncBool" => {
            let result = CallFuncBoolCallback(mock_bool);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncChar8" => {
            let result = CallFuncChar8Callback(mock_char8);
            ReverseReturn(&PlgString::from(format!("{}", result as u8)));
        }
        "CallFuncChar16" => {
            let result = CallFuncChar16Callback(mock_char16);
            ReverseReturn(&PlgString::from(format!("{}", result as i16)));
        }
        "CallFuncInt8" => {
            let result = CallFuncInt8Callback(mock_int8);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncInt16" => {
            let result = CallFuncInt16Callback(mock_int16);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncInt32" => {
            let result = CallFuncInt32Callback(mock_int32);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncInt64" => {
            let result = CallFuncInt64Callback(mock_int64);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncUInt8" => {
            let result = CallFuncUInt8Callback(mock_uint8);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncUInt16" => {
            let result = CallFuncUInt16Callback(mock_uint16);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncUInt32" => {
            let result = CallFuncUInt32Callback(mock_uint32);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncUInt64" => {
            let result = CallFuncUInt64Callback(mock_uint64);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncPtr" => {
            let result = CallFuncPtrCallback(mock_ptr);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncFloat" => {
            let result = CallFuncFloatCallback(mock_float);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncDouble" => {
            let result = CallFuncDoubleCallback(mock_double);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFuncString" => {
            let result = CallFuncStringCallback(mock_string);
            ReverseReturn(&result);
        }
        "CallFuncAny" => {
            let result = CallFuncAnyCallback(mock_any);
            ReverseReturn(&PlgString::from(format!("{:?}", result.get())));
        }
        "CallFuncBoolVector" => {
            let result = CallFuncBoolVectorCallback(mock_bool_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncChar8Vector" => {
            let result = CallFuncChar8VectorCallback(mock_char8_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncChar16Vector" => {
            let result = CallFuncChar16VectorCallback(mock_char16_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncInt8Vector" => {
            let result = CallFuncInt8VectorCallback(mock_int8_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncInt16Vector" => {
            let result = CallFuncInt16VectorCallback(mock_int16_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncInt32Vector" => {
            let result = CallFuncInt32VectorCallback(mock_int32_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncInt64Vector" => {
            let result = CallFuncInt64VectorCallback(mock_int64_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncUInt8Vector" => {
            let result = CallFuncUInt8VectorCallback(mock_uint8_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncUInt16Vector" => {
            let result = CallFuncUInt16VectorCallback(mock_uint16_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncUInt32Vector" => {
            let result = CallFuncUInt32VectorCallback(mock_uint32_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncUInt64Vector" => {
            let result = CallFuncUInt64VectorCallback(mock_uint64_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncPtrVector" => {
            let result = CallFuncPtrVectorCallback(mock_ptr_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncFloatVector" => {
            let result = CallFuncFloatVectorCallback(mock_float_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncDoubleVector" => {
            let result = CallFuncDoubleVectorCallback(mock_double_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncStringVector" => {
            let result = CallFuncStringVectorCallback(mock_string_vector);
            let strings: Vec<String> = result.iter().map(|s| s.as_str().to_string()).collect();
            ReverseReturn(&PlgString::from(format!("{:?}", strings)));
        }
        "CallFuncAnyVector" => {
            let result = CallFuncAnyVectorCallback(mock_any_vector);
            let anys: Vec<String> = result.iter().map(|a| format!("{:?}", a.get())).collect();
            ReverseReturn(&PlgString::from(format!("{:?}", anys)));
        }
        "CallFuncVec2Vector" => {
            let result = CallFuncVec2VectorCallback(mock_vec2_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncVec3Vector" => {
            let result = CallFuncVec3VectorCallback(mock_vec3_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncVec4Vector" => {
            let result = CallFuncVec4VectorCallback(mock_vec4_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncMat4x4Vector" => {
            let result = CallFuncMat4x4VectorCallback(mock_mat4x4_vector);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncVec2" => {
            let result = CallFuncVec2Callback(mock_vec2);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncVec3" => {
            let result = CallFuncVec3Callback(mock_vec3);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncVec4" => {
            let result = CallFuncVec4Callback(mock_vec4);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFuncMat4x4" => {
            let result = CallFuncMat4x4Callback(mock_mat4x4);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFunc1" => {
            let result = CallFunc1Callback(mock_func1);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFunc2" => {
            let result = CallFunc2Callback(mock_func2);
            ReverseReturn(&PlgString::from(format!("{}", result as u8)));
        }
        "CallFunc3" => {
            CallFunc3Callback(mock_func3);
        }
        "CallFunc4" => {
            let result = CallFunc4Callback(mock_func4);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFunc5" => {
            let result = CallFunc5Callback(mock_func5);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFunc6" => {
            let result = CallFunc6Callback(mock_func6);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFunc7" => {
            let result = CallFunc7Callback(mock_func7);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFunc8" => {
            let result = CallFunc8Callback(mock_func8);
            ReverseReturn(&PlgString::from(format!("{:?}", result)));
        }
        "CallFunc9" => {
            CallFunc9Callback(mock_func9);
        }
        "CallFunc10" => {
            let result = CallFunc10Callback(mock_func10);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFunc11" => {
            let result = CallFunc11Callback(mock_func11);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFunc12" => {
            let result = CallFunc12Callback(mock_func12);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFunc13" => {
            let result = CallFunc13Callback(mock_func13);
            ReverseReturn(&result);
        }
        "CallFunc14" => {
            let result = CallFunc14Callback(mock_func14);
            let strings: Vec<String> = result.iter().map(|s| s.as_str().to_string()).collect();
            ReverseReturn(&PlgString::from(format!("{:?}", strings)));
        }
        "CallFunc15" => {
            let result = CallFunc15Callback(mock_func15);
            ReverseReturn(&PlgString::from(format!("{}", result)));
        }
        "CallFunc16" => {
            let result = CallFunc16Callback(mock_func16);
            ReverseReturn(&PlgString::from(format!("{}", result)));
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
        _ => {
            // Unknown test name, do nothing
        }
    }
}