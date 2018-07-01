//! LLVM utilities, mostly for strings.

#![allow(dead_code)]
// Code here should be cleaned up and used.
// https://github.com/immington-industries/protosnirk/issues/77

use std::ffi::CStr;

use libc::{c_char};

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct StringWithDrop {
    inner: String
}

pub fn str_to_char_star(input: &str) -> *const c_char {
    input.as_ptr() as *const c_char
}

pub fn str_from_char_star(input: *const c_char) -> String {
    if input.is_null() {
        String::from("")
    }
    else {
        unsafe {
            CStr::from_ptr(input).to_string_lossy().into_owned()
        }
    }
}
