use alpm_rs;

#[test]
fn sync_dbs(){
    let handle = alpm_rs::initialize("/", "/var/lib/pacman").unwrap();
    let dbs = handle.sync_dbs();

    match handle.error_no(){
       alpm_rs::enums::ErrorNo::ALPM_ERR_OK => {},
       _ => panic!("last error not ok"),
   }
}