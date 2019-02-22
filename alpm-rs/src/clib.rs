



use std::error::Error;
use std::ffi::CString;
use std::os::raw::c_char;

#[link(name="alpm")]
extern {
    fn vsnprintf(n: *mut [c_char], fmt: *const c_char, args: va_list::VaList) -> i32;
}
/*
fn vsn_printf(n: *mut c_char, fmt: &mut c_char, mut args: va_list::VaList) -> String{
    let  mut sized buf : [u8];


    let len = vsnprintf(ptr!(buf, c_char), ptr!(fmt, c_char), args);

    if len > buf.capacity(){
        buf = [c_char;len];
        vsnprintf(ptr!(buf, c_char), ptr!(fmt, c_char), args);
    }

    cstr!(buf)
}*/