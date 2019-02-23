
#[test]
fn test_callbacks(){
    let handle = alpm_rs::initialize("/", "/var/lib/pacman").unwrap();

    alpm_rs::callbacks::set_log_callback(&handle, |l,m| println!("{}] {}", l, &m));
    alpm_rs::callbacks::set_download_callback(&handle, |f,x,t| println!("{},{},{}", f,x,t));

    handle.register_syncdb("core", 0);
    handle.register_syncdb("community", 0);
    let dbs = handle.sync_dbs();

    match handle.error_no(){
       alpm_rs::enums::ErrorNo::ALPM_ERR_OK => {},
       _ => panic!("last error not ok"),
   }

    
    for db in dbs {
        for _ in db.search_one("git"){
        }
    }
    
}