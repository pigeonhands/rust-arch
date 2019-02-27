# alpm-rs
alpm bindings for rust.

[![alpm-rs](https://img.shields.io/crates/v/alpm-rs.svg?label=crates.io)](https://crates.io/crates/alpm-rs)

Usage:

[kia is made with this crate](https://github.com/BahNahNah/kea)

```
let handle = alpm_rs::initialize("/", "/var/lib/pacman").unwrap();
let db = handle.local_db();
let packages = db.pkgcache();

for p in packages {
    println!("{} {}", p.name(), p.version());
}
```

[alpm.h reference](https://github.com/devkitPro/pacman/blob/master/lib/libalpm/alpm.h)