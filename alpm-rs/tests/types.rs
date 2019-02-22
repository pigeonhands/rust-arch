use alpm_rs::types::{StringList, AlpmListType, AlpmListItem};
use std::os::raw::c_char;


#[test]
fn test_str_list(){
    let items = &["1", "2", "3"];

    let mut needle_list = StringList::empty();
    for n in items{
        needle_list.add((*n).into());
    }

    let mut iter = needle_list.iter();
    unsafe{
        for _ in items{
            match iter.next() {
                None => assert_eq!(true,false),
                Some(s) => println!("{}",  std::ffi::CStr::from_ptr(s.to_ptr() as *const c_char).to_str().unwrap_or("")),
            }
        }
    }
}