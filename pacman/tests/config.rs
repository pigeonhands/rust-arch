use pacman::config::PacmanConfig;

#[test]
fn pacman_conf() {
    let cfg = pacman::pacman_conf::get_config();
    assert_ne!(cfg.root_dir, "");
}

#[test]
fn pacman_cfg_parse(){
    let raw_cfg = r#"
[options]
Color
RootDir = /rootdir
UseSyslog
Color
UseDelta = 1.1
TotalDownload
CheckSpace
VerbosePkgLists
DisableDownloadTimeout


[core]
Server = s1
Server = s2
SigLevel = Never
Usage = use

Include = testdata/custom
    "#;

    let cfg  = PacmanConfig::parse(raw_cfg);
    assert_eq!(cfg.root_dir, "/rootdir");
    assert_eq!(cfg.color, true);

    let core = cfg.repo("core").unwrap();
    assert_eq!(core.servers.len(), 2);
    assert_eq!(core.servers[0], "s1");
    assert_eq!(core.servers[1], "s2");
}