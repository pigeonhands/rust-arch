use alpm_rs;

#[test]
fn sync_dbs(){
    let handle = alpm_rs::initialize("/", "/var/lib/pacman").unwrap();
    let db = handle.local_db();

    match handle.error_no(){
       alpm_rs::enums::ErrorNo::ALPM_ERR_OK => {},
       _ => panic!("last error not ok"),
   }

   let pkg = db.pkgcache().iter().skip(3).next().unwrap();
   assert_ne!(pkg.name(), "");
}