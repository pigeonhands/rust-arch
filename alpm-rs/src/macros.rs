/// string to c-string
macro_rules! strc {
    ($e:expr) => {
        std::ffi::CString::new($e).unwrap().as_ptr()
    };
}

/// c-string to string
macro_rules! cstr {
    ($e:expr) => {
        if $e == std::ptr::null_mut() {
                ""
            }else{
                std::ffi::CStr::from_ptr($e).to_str().unwrap_or("")
            }
    };
}

macro_rules! ptr {
    ($e:expr, $t:ty) => {
        $e as *mut $t
    };
}

macro_rules! to_bool {
    ($e:expr) => {
        if $e == -1 { false } else { true }
    };
}