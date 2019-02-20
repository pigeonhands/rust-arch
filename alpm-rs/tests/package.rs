use alpm_rs;

#[test]
fn pkg_cache() {
    let handle = alpm_rs::initialize("/", "/var/lib/pacman").unwrap();
    let db = handle.register_syncdb("core", 0);
    let list = db.pkgcache();

    match handle.error_no(){
       alpm_rs::enums::ErrorNo::ALPM_ERR_OK => {},
       _ => panic!("last error not ok"),
   }

    for p in list {
        assert_ne!(p.filename(), "");
        assert_ne!(p.base64_sig(), "");
        assert_ne!(p.name(), "");
        assert_ne!(p.version(), "");
        assert_ne!(p.url(), "");
    }
}

#[test]
fn local_cashe(){
    let handle = alpm_rs::initialize("/", "/var/lib/pacman").unwrap();
    let db = handle.local_db();
    let list = db.pkgcache();

   match handle.error_no(){
       alpm_rs::enums::ErrorNo::ALPM_ERR_OK => {},
       _ => panic!("last error not ok"),
   }

    for p in list {
        assert_ne!(p.version(), "");
    }

}