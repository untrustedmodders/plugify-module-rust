// Generated from cross_call_master.pplugin

#[allow(unused_imports)]
use plugify::{vector_enum_traits};

#[repr(i32)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Example {
    First = 1,
    Second = 2,
    Third = 3,
    Forth = 4,
}
vector_enum_traits!(Example, i32);


/// Ownership type for RAII wrappers
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ownership {
    Borrowed,
    Owned,
}
