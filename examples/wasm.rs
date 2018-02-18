/// Mainly based on
/// https://www.hellorust.com/demos/sha1/index.html

extern crate geopattern;

use std::ffi::{CString, CStr};
use std::mem;
use std::os::raw::{c_char, c_void};
use std::str;

#[no_mangle]
pub fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr as *mut c_void
}

#[no_mangle]
pub fn dealloc(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub fn geopattern_length(s: *mut c_char) -> u32 {
    unsafe {
        let data = CStr::from_ptr(s);
        let s = str::from_utf8(data.to_bytes()).unwrap();
        let gp = geopattern::generate(s);
        let gp = gp.to_svg().unwrap().to_string();
        count_newlines(&gp) as u32
    }
}

fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

#[no_mangle]
pub fn geopattern_line(s: *mut c_char, i: u32) -> *mut c_char {
    unsafe {
        let data = CStr::from_ptr(s);
        let s = str::from_utf8(data.to_bytes()).unwrap();
        let gp = geopattern::generate(s);
        let gp = gp.to_svg().unwrap().to_string();
        let line = gp.lines().nth(i as usize).unwrap();
        let s = CString::new(line).unwrap();
        s.into_raw()
    }
}

#[no_mangle]
pub fn geopattern(s: *mut c_char) -> *mut c_char {
    unsafe {
        let data = CStr::from_ptr(s);
        let s = str::from_utf8(data.to_bytes()).unwrap();
        let gp = geopattern::generate(s);
        let gp = gp.to_base64().unwrap().to_string();
        let s = CString::new(gp).unwrap();
        s.into_raw()
    }
}

fn main() {
    // intentionally left blank
}
