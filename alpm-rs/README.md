# alpm-rs
alpm bindings for rust.

## usage


    [dependencies]
    alpm_rc = "*"


Usage:

    let handle = alpm_rs::initialize("/", "/var/lib/pacman").unwrap();
        let db = handle.local_db();
        let packages = db.pkgcache();
    }

    for p in packages {
        println!("{} {}", p.name(), p.version());
    }


references:
`https://github.com/devkitPro/pacman/blob/master/lib/libalpm/alpm.h`