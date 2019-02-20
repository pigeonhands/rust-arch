use aur_client::aur;

#[test]
fn search() {
    let resp = aur::search("python3").unwrap();
    assert_eq!(resp.error, None);
}

#[test]
fn info(){
    let resp = aur::info(&["python3"]).unwrap();
    assert_eq!(resp.error, None);
}