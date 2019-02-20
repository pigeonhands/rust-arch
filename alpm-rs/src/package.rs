use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr;

use crate::enums::PkgFrom;
use crate::types::{alpm_list_t, AlpmList,AmplListItem};

#[link(name="alpm")]
extern {
    fn alpm_pkg_find(pkg_list: *mut alpm_list_t, haystack: *const c_char) -> *mut alpm_pkg_t ;

    fn alpm_pkg_free(pkg: *mut alpm_pkg_t) -> i32;
    fn alpm_pkg_vercmp(a: *const c_char, b: *const c_char) -> i32;
    fn alpm_pkg_checkmd5sum(pkg: *mut alpm_pkg_t) -> i32;
    
    fn alpm_pkg_get_filename(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_base(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_base64_sig(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_name(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_version(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_origin(pkg: *mut alpm_pkg_t) -> PkgFrom;
    fn alpm_pkg_get_desc(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_get_url(pkg: *mut alpm_pkg_t) -> *const c_char;
    fn alpm_pkg_compute_requiredby(pkg: *mut alpm_pkg_t)-> *mut alpm_list_t;
    fn alpm_pkg_compute_optionalfor(pkg: *mut alpm_pkg_t)-> *mut alpm_list_t;
    fn alpm_pkg_get_validation(pkg: *mut alpm_pkg_t) -> i32;
}

pub type PackageList = AlpmList<Package>;

#[repr(C)]
pub struct alpm_pkg_t{
    pub (crate) __unused: [u8;0],//get rid of warnings
}


pub struct Package {
    pub (crate) pkg: *mut alpm_pkg_t,
}

impl AmplListItem<Package> for Package{
    fn new(data_ptr: *mut c_void) -> Package{
        Package::from_c_pkg(data_ptr as *mut alpm_pkg_t)
    }
}

impl PackageList {
    pub fn find(&self, haystack: &str) -> Package{
        let pkg = callc!(alpm_pkg_find(self.list, strc!(haystack)));
        Package::from_c_pkg(pkg)
    }
}


impl Drop for Package {
    fn drop(&mut self) {
        self.free();
    }
}

impl Package{
    pub (crate) fn from_c_pkg(c_pkg: *mut alpm_pkg_t) -> Package{
        Package{
            pkg: c_pkg,
        }
    }

    /// Compare two version strings and determine which one is 'newer'. 
    pub fn vercmp(a : &str, b :&str) -> i32 {
        callc!(alpm_pkg_vercmp(
            strc!(a),
            strc!(b)
        ))
    }

    /// Returns the method used to validate a package during install.
    /// Flags:
    /// `
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_UNKNOWN
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_NONE 
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_MD5SUM
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_SHA256SUM
    /// alpm_rs::enum::ALPM_PKG_VALIDATION_SIGNATURE
    /// `
    pub fn validation(&self) -> i32{
        callc!(
            alpm_pkg_get_validation(self.pkg)
        )
    }
    /// Gets the name of the file from which the package was loaded.
    pub fn filename(&self) -> &str {
        cstr!(alpm_pkg_get_filename(self.pkg))
    }

    /// Returns the package base name.
    pub fn base(&self) -> &str {
        cstr!(alpm_pkg_get_base(self.pkg))
    }

    /// Returns the package name.
    pub fn base64_sig(&self) -> &str {
        cstr!(alpm_pkg_get_base64_sig(self.pkg))
    }

    /// Returns the package name.
    pub fn name(&self) -> &str {
        cstr!(alpm_pkg_get_name(self.pkg))
    }

    /// Returns the package version as a string.
    /// This includes all available epoch, version, and pkgrel components. 
    /// Use vercmp() to compare version strings if necessary.
    pub fn version(&self) -> &str {
        cstr!(alpm_pkg_get_version(self.pkg)) 
    }

    /// Returns the origin of the package.
    pub fn origin(&self) -> PkgFrom {
        callc!(alpm_pkg_get_origin(self.pkg))
    }

    /// Returns the package description.
    pub fn description(&self) -> &str {
        cstr!(alpm_pkg_get_desc(self.pkg)) 
    }

    /// Returns the package URL.
    pub fn url(&self) -> &str {
        cstr!(alpm_pkg_get_url(self.pkg))
    }

    /// Computes the list of packages requiring a given package.
    pub fn required_by(&self) -> PackageList {
        let pl = callc!(alpm_pkg_compute_requiredby(self.pkg));
        PackageList::new(pl)
    } 

    /// Computes the list of packages optionally requiring a given package.
    pub fn optional_for(&self) -> PackageList {
        let pl = callc!(alpm_pkg_compute_optionalfor(self.pkg));
        PackageList::new(pl)
    }    

    /// Check the integrity (with md5) of a package from the sync cache.
    pub fn checkmd5sum(&self) -> bool {
        to_bool!(
            callc!(alpm_pkg_checkmd5sum(self.pkg))
        )
    }

    /// Free package
    pub fn free(&self) -> bool {
        to_bool!(
            callc!(alpm_pkg_free(self.pkg))
        )
    }
}