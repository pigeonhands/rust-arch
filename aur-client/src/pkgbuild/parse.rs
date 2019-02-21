

#[derive(Debug)]
struct Pkgbuild {
    field: Type
}

fn (tkns: &TokenList){
    pkgname: Vec<string>,
    pkgver: String,
    pkgrel: (u32, Option<u32>),
    epoch: u32,
    pkgdesc: String,
    url: String,
    license: Vec<String>,
    install: String,
}