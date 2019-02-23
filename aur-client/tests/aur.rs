use aur_client::aur;

#[test]
fn search() {
    let resp = aur::search("python3").unwrap();
    assert_eq!(resp.error, None);
}

#[test]
fn info(){
    let resp = aur::info(&["spotify"]).unwrap();
    assert_eq!(resp.error, None);
    assert_ne!(resp.results.len(), 0);
}