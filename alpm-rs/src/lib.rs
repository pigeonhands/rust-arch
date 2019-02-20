
#[macro_use]
pub mod macros;


pub mod error;
pub mod enums;
pub mod db;
pub mod package;
pub mod types;

extern crate libc;

use std::error::Error;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::package::{Package,alpm_pkg_t};
use crate::types::{alpm_list_t, AmplListItem};
use crate::db::{AlpmDB,alpm_db_t, DBList};
use crate::enums::ErrorNo;

#[link(name="alpm")]
extern {
    fn alpm_initialize(root: *const c_char, dbpath: *const c_char, error: *mut ErrorNo) -> *mut alpm_handle_t;
    fn alpm_release(handle : *mut alpm_handle_t) -> i32;
    fn alpm_errno(handle : *mut alpm_handle_t) -> ErrorNo;
    fn alpm_pkg_should_ignore(handle : *mut alpm_handle_t, pkg_handle:  *mut alpm_pkg_t) -> i32;
    fn alpm_pkg_load(handle : *mut alpm_handle_t, filename: *const c_char, full : i32, level: i32, pkg: *mut alpm_pkg_t) -> i32;

    fn alpm_get_localdb(handle : *mut alpm_handle_t) -> *mut alpm_db_t;
    fn alpm_get_syncdbs(handle : *mut alpm_handle_t) -> *mut alpm_list_t;

    fn alpm_unregister_all_syncdbs(handle : *mut alpm_handle_t) -> i32;
    
    fn alpm_register_syncdb(handle : *mut alpm_handle_t, treename : *const c_char, level: i32 ) -> *mut alpm_db_t;

}


#[repr(C)]
struct alpm_handle_t{
     __unused: [u8;0],//get rid of warnings

}

/// Alpm handle
pub struct Handle {
    alpm_handle: *mut alpm_handle_t,
}

 /// Initializes the alpm library and retuns a handle.
 /// ///
 /// /// # Arguments
 /// ///
 /// /// * `root` - the root path for all filesystem operations
 /// /// * `dbpath` - the absolute path to the libalpm database
 /// ///
 /// /// # Example
 /// ///
 /// /// ```
 /// /// use alpm;
 /// /// let handle = alpm::initialize("/", "/var/lib/pacman").unwrap();
 /// /// ```
pub fn initialize(root: &str, dbpath: &str) -> Result<Handle, Box<dyn Error>> {
    let mut err = enums::ErrorNo::ALPM_ERR_OK;
    let handle : *mut alpm_handle_t;

    unsafe{
        handle = alpm_initialize(
                    strc!(root), 
                    strc!(dbpath), 
                    &mut err as *mut enums::ErrorNo);
    }
            
    if err as i32 != 0{
        Err(error::AlpmError::new(err).into())
    }else{
        Ok(Handle{
            alpm_handle: handle,
        })
    }
}

impl Handle{

    /// Get the database of locally installed packages.
    pub fn local_db(&self) -> AlpmDB{
        AlpmDB {
            db: callc!(alpm_get_localdb(self.alpm_handle)),
        }
    }

    /// Get the list of sync databases.
    pub fn sync_dbs(&self) -> DBList{
        DBList::new(
            callc!(alpm_get_syncdbs(self.alpm_handle))
        )
    }

    ///  Register a sync database of packages.
    pub fn register_syncdb(&self, tree_name : &str, level: i32) -> AlpmDB {
        AlpmDB{
            db: callc!(alpm_register_syncdb(self.alpm_handle, strc!(tree_name), level)),
        }
    }

    /// Unregister all package databases.
    pub fn unregister_all_syncdbs(&self) -> bool {
        to_bool!(
            callc!(alpm_unregister_all_syncdbs(self.alpm_handle))
        )
    }

    /// Release handle
    pub fn release(&self) -> i32 {
        unsafe{
            alpm_release(self.alpm_handle)
        }
    }

    /// Returns last error code
    pub fn error_no(&self) -> ErrorNo {
        callc!(alpm_errno(self.alpm_handle))
    }

    /// Test if a package should be ignored.
    /// Checks if the package is ignored via IgnorePkg, or if the package is
    /// in a group ignored via IgnoreGroup.
    pub fn should_ignore(&self, pkg : &Package) -> bool{
        to_bool!(
            callc!(alpm_pkg_should_ignore(self.alpm_handle, pkg.pkg))
        )
    }

    /// Create a package from a file.
    /// If full is false, the archive is read only until all necessary
    /// metadata is found. If it is true, the entire archive is read, which
    /// serves as a verification of integrity and the filelist can be created.
    ///  /// /// # Arguments
    /// ///
    /// /// * `filename` - location of the package tarball
    /// /// * `full` - whether to stop the load after metadata is read or continue through the full archive
    /// /// * `level` - what level of package signature checking to perform on the package; note that this must be a '.sig' file type verification
    /// ///
    pub fn load_package(&self, filename: &str, full: bool, level: i32) -> Option<Package>{
        let mut lm_pkg =  Box::new(alpm_pkg_t{
            __unused: [],
        });
       let pkg_ptr = &mut *lm_pkg as *mut alpm_pkg_t;

        let err = callc!(alpm_pkg_load(
                self.alpm_handle,
                strc!(filename),
                full.into(),
                level,
                 pkg_ptr,// as alpm_pkg_t,
            ));
        if err != -1{
            Some(Package::from_c_pkg(pkg_ptr))
        }else{
            None
        }
    }
}

