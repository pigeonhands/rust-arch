use std::os::raw::{c_void};
use std::ptr;
use std::marker::PhantomData;

#[link(name="alpm")]
extern {
    fn alpm_list_free(list: *mut alpm_list_t);
}

#[repr(C)]
pub (crate) struct alpm_list_t  {
	data : *mut c_void,
	prev: *mut alpm_list_t,
	next: *mut alpm_list_t,
}

impl<T: AmplListItem<T>> From<*mut alpm_list_t> for AlpmList<T> {
    fn from(c_list: *mut alpm_list_t) -> AlpmList<T> {
        AlpmList::new(c_list)
    }
}

/// Trait to pass each item into a custom struct's constructor.
/// Workaround to avoid copying data and loosing the alpm_list_t data pointer 
pub trait AmplListItem<T>{
    fn new(data_ptr: *mut c_void) -> Self;
}

/// Generic almp list
pub struct AlpmList<T> {
    pub (crate) list: *mut alpm_list_t,
    current: *mut alpm_list_t,
    phantom: PhantomData<T>,
}

impl<T: AmplListItem<T>> AlpmList<T> {
    pub (crate) fn new(c_list: *mut alpm_list_t) -> AlpmList<T> {
        AlpmList {
            list: c_list,
            current: c_list,
            phantom:PhantomData,
        }
    }
}

impl<T> Drop for AlpmList<T> {
    fn drop(&mut self) {
        unsafe{
            alpm_list_free(self.list);
        }
    }
}

 impl<T: AmplListItem<T>> std::iter::Iterator for AlpmList<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current == ptr::null_mut(){
            return None;
        }
        unsafe{
            if (*self.current).next != ptr::null_mut() {
                self.current = (*self.current).next;
                Some(T::new((*self.current).data ))
            } else {
                None
            }
        }
    }
 }