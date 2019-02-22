


use std::os::raw::c_char;

#[link(name="alpm")]
extern {
    fn vsnprintf(s: *mut c_char, num: usize,fmt: *mut i32, args: va_list::VaList) -> i32;
}

pub unsafe fn vsn_printf(fmt: *mut c_char, args: va_list::VaList) -> String{

    let mut buf = vec![0;300];
    println!("0] {}", cstr!(fmt));

    let mut testBuff = [0;666];

    let len = vsnprintf(fmt, 0, testBuff.as_mut_ptr(), args) as usize;
    println!("1] {}", len);
    if len > buf.len(){
        buf = vec![0;len + 1].into();
      //  vsnprintf(fmt, buf.len(), ptr!(buf.as_mut_ptr(),c_char), args);
    }
        println!("2] {}", len);


    cstr!(buf.as_mut_ptr() as *mut c_char).to_string()
}