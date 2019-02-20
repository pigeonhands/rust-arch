search

    use aur_client::aur;
    let resp = aur::search("python3").unwrap();


download

    aur::clone("python3", Path::new("python3"))?;