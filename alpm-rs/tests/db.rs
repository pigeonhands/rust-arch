use alpm_rs::AlpmListType;

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


#[test]
fn search_db(){
    let handle = alpm_rs::initialize("/", "/var/lib/pacman").unwrap();
    let db = handle.register_syncdb("extra", 0);

    match handle.error_no(){
       alpm_rs::enums::ErrorNo::ALPM_ERR_OK => {},
       _ => panic!("last error not ok"),
   }

    let search = db.search(&["python*"]);
   assert_ne!( search.iter().count(), 0);
}