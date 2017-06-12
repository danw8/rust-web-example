use libc;
use std::str;
use std::ffi::{CString, CStr};

pub trait Interop {
    fn as_int(self, _:&mut Vec<CString>) -> libc::c_int;
}

impl<'a> Interop for &'a str {
    fn as_int(self, arena:&mut Vec<CString>) -> libc::c_int {
        let c = CString::new(self).unwrap();
        let ret = c.as_ptr() as libc::c_int;
        arena.push(c);
        return ret;
    }
}

pub fn http_get(url: &str) -> String{
    let a = js!{ (url) b"\
            var xmlHttp = new XMLHttpRequest();\
            xmlHttp.open( 'GET', UTF8ToString($0), false );\
            xmlHttp.send( null );\
            return allocate(intArrayFromString(xmlHttp.responseText), 'i8', ALLOC_STACK);\
        \0" };
    println!("{:?}", a);
    unsafe {
        str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
    }
}
