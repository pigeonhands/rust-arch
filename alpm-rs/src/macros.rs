// for consistancy
macro_rules! callc{
    ($e:expr) => {
        unsafe{
            $e
        }
    };
}

/// string to c-string
macro_rules! strc {
    ($e:expr) => {
        callc!(
            CString::new($e).unwrap().as_ptr()
        )
    };
}

/// c-string to string
macro_rules! cstr {
    ($e:expr) => {
        callc!(
            if $e == ptr::null_mut() {
                ""
            }else{
                CStr::from_ptr($e).to_str().unwrap_or("")
            }
        )
    };
}

macro_rules! to_bool {
    ($e:expr) => {
        if $e == -1 { false } else { true }
    };
}