use std::os::raw::c_char;

use super::{Handle,alpm_handle_t};
use crate::clib;

#[link(name="alpm")]
extern{
    fn alpm_option_set_logcb(handle : *mut alpm_handle_t, f: extern fn(i32, *mut c_char, va_list::VaList)) -> i32;
}


type LogCallback = fn(i32,String);

static mut LOG_CALLBACK : Option<LogCallback> = None;

extern "C" fn alpm_log_cb_handle(level: i32, fmt: *mut c_char, args: va_list::VaList){
        unsafe{
            if let Some(f) = LOG_CALLBACK{
            let out = clib::vsn_printf(fmt, args);
            f(level, out);
        }
    }
}

pub fn set_log_callback(h: &Handle, cb: LogCallback){
    unsafe{
        match LOG_CALLBACK{
            None => {
                alpm_option_set_logcb(h.alpm_handle, alpm_log_cb_handle);
            },
            _ => {},
        }
        LOG_CALLBACK = Some(cb);
    }
}