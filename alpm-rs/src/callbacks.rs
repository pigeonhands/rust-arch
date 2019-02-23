use std::os::raw::c_char;

use super::{Handle,alpm_handle_t};
use crate::clib;

#[link(name="alpm")]
extern{
    fn alpm_option_set_logcb(handle : *mut alpm_handle_t, f: extern fn(i32, *mut c_char, clib::VaList)) -> i32;

    fn alpm_option_set_dlcb(handle : *mut alpm_handle_t,f: extern fn(filename: *mut c_char, xfered: i64, total: i64)) -> i32;

}

type LogCallback = fn(i32,String);
type DownloadCallback = fn(filename: &str, downloaded: i64, total: i64);


static mut LOG_CALLBACK : Option<LogCallback> = None;
static mut DOWNLOAD_CALLBACK : Option<DownloadCallback> = None;


extern "C" fn alpm_log_cb_handler(level: i32, fmt: *mut c_char, args: clib::VaList){
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
                alpm_option_set_logcb(h.alpm_handle, alpm_log_cb_handler);
            },
            _ => {},
        }
        LOG_CALLBACK = Some(cb);
    }
}

extern "C" fn alpm_log_download_handler(filename: *mut c_char, xfered: i64, total: i64){
    unsafe{
        if let Some(f) = DOWNLOAD_CALLBACK{
            f(cstr!(filename), xfered, total);
        }
    }
}

pub fn set_download_callback(h: &Handle, cb: DownloadCallback){
    unsafe{
        match DOWNLOAD_CALLBACK{
            None => {
                alpm_option_set_dlcb(h.alpm_handle, alpm_log_download_handler);
            },
            _ => {},
        }
        DOWNLOAD_CALLBACK = Some(cb);
    }
}