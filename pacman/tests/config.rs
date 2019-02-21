

#[test]
fn parse_config(){
    let raw_cfg = r#"
[options]
RootDir = /path/to/root/dir
DBPath = /path/to/db/dir
CacheDir = /path/to/cache/dir
HookDir = /path/to/hook/dir
GPGDir = /path/to/gpg/dir
LogFile = /path/to/log/file
HoldPkg = hold package
IgnorePkg = ignore package
IgnoreGroup = ignore group
Architecture = neo-classical
XferCommand = /path/to/command %u
NoUpgrade = no upgrade
NoExtract = no extract
CleanMethod = KeepInstalled KeepCurrent
SigLevel = PackageRequired
SigLevel = PackageTrustedOnly
SigLevel = DatabaseRequired
SigLevel = DatabaseTrustedOnly
LocalFileSigLevel = PackageOptional
LocalFileSigLevel = PackageTrustedOnly
RemoteFileSigLevel = PackageNever
UseSyslog
Color
UseDelta = 1.1
TotalDownload
CheckSpace
VerbosePkgLists
DisableDownloadTimeout

[repo1]
Server = foo
Server = bar
SigLevel = Never
Usage = Sync Search

Include = testdata/custom
    "#;
}