extern crate libc;

use std::ffi::{CStr,CString};
use std::os::raw::{c_char,c_void};
use std::ptr;

use crate::{Handle, alpm_handle_t};
use crate::package::{PackageList};
use crate::types::{alpm_list_t, AlpmList, AmplListItem};


#[link(name="alpm")]
extern {
    fn alpm_db_get_pkgcache(db: *mut alpm_db_t)-> *mut alpm_list_t;
    fn alpm_db_unregister(db: *mut alpm_db_t) -> i32;
    fn alpm_db_get_name(db: *mut alpm_db_t) -> *const c_char;
}


#[repr(C)]
pub (crate) struct alpm_db_t{
    __unused: [u8;0],//get rid of warnings

}

/// Alpm database
pub struct AlpmDB {
    pub (crate) db: *mut alpm_db_t,
}

impl AlpmDB{

    /// Get the name of the package database.
    pub fn name(&self) -> &str {
        cstr!(
            alpm_db_get_name(self.db)
        )
    }

    /// Get the package cache of the package database.
    pub fn pkgcache(&self) -> PackageList {
        let pl = callc!(alpm_db_get_pkgcache(self.db));
        PackageList::new(pl)
    }

    /// Unregister the package database.
    pub fn unregister(&self) -> bool{
        to_bool!(
            callc!(alpm_db_unregister(self.db))
        )
    }
}

impl Drop for AlpmDB {
    fn drop(&mut self) {
    }
}

pub type DBList = AlpmList<AlpmDB>;

impl AmplListItem<AlpmDB> for AlpmDB{
    fn new(data_ptr: *mut c_void) -> AlpmDB{
        AlpmDB{
            db: data_ptr as *mut alpm_db_t,
        }
    }
}