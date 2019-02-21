use ini::Ini;

pub struct PacmanConfig {
    root_dir: String,
    db_path: String,
    cashe_dir: Vec<String>,
    hook_dir: Vec<String>,
    log_file: String,
    hold_pkg: Vec<String>,
    ignore_pkg: Vec<String>,
    ignore_group: Vec<String>,
    architecture: String,
    xfer_command: String,
    no_upgrade: Vec<String>,
    no_extract: Vec<String>,
    clean_method: Vec<String>,
    sig_level: Vec<String>,
    local_file_sig_level: Vec<String>,
    remote_file_sig_level: Vec<String>,
    use_syslog: bool,
    color: bool,
    use_delta: f64,
    total_download: bool,
    check_space:bool,
    verbose_pkg_list: bool,
    disable_download_timeout: bool,
    repos: Vec<Repository>,
}

pub struct Repository {
    name: String,
    servers: Vec<String>,
    sig_level: Vec<String>,
    usage: Vec<String>,
}

#[allow(unconditional_recursion)]
impl Default for Repository{
    fn default() -> Self{
        Repository{
            ..Default::default()
        }
    }
}

#[allow(unconditional_recursion)]
impl Default for PacmanConfig{
    fn default() -> Self{
        PacmanConfig{
            ..Default::default()
        }
    }
}

impl PacmanConfig{
    pub fn parse(cfg: String) -> Self{
        let i = Ini::load_from_str(&cfg).unwrap_or_else(|_| Ini::new());

        let mut cfg = PacmanConfig::default();

        for (sec, prop) in i.iter() {
            let mut section : String;
            match sec{
                Some(s) => section = s.to_string(),
               _ => section = "".to_string(),
            }
            match section.as_ref() {
                "options" => {
                    for (k, v) in prop.iter() {
                        match k.as_ref() {
                            "RootDir"=> cfg.root_dir= v.to_string(),
                            "DbPath"=> cfg.db_path= v.to_string(),
                            "CasheDir"=> cfg.cashe_dir.push(v.to_string()),
                            "HookDir"=> cfg.hook_dir.push(v.to_string()),
                            "LogFile"=> cfg.log_file= v.to_string(),
                            "HoldPkg"=> cfg.hold_pkg.push(v.to_string()),
                            "IgnorePkg"=> cfg.ignore_pkg.push(v.to_string()),
                            "IgnoreGroup"=> cfg.ignore_group.push(v.to_string()),
                            "Architecture"=> cfg.architecture= v.to_string(),
                            "XferCommand"=> cfg.xfer_command =v.to_string(),
                            "NoUpgrade"=> cfg.no_upgrade.push(v.to_string()),
                            "NoExtract"=> cfg.no_extract.push(v.to_string()),
                            "CleanMethod"=> cfg.clean_method.push(v.to_string()),
                            "SigLevel"=> cfg.sig_level.push(v.to_string()),
                            "LocalFileSigLevel"=> cfg.local_file_sig_level.push(v.to_string()),
                            "RemoteFileSigLevel"=> cfg.remote_file_sig_level.push(v.to_string()),
                            "UseSyslog"=> cfg.use_syslog = v.parse().unwrap_or(false),
                            "Color"=> cfg.color = v.parse().unwrap_or(false),
                            "UseDelta"=> cfg.use_delta = v.parse().unwrap_or(0.0),
                            "TotalDownload"=> cfg.total_download = v.parse().unwrap_or(false),
                            "CheckSpace"=> cfg.check_space = v.parse().unwrap_or(false),
                            "VerbosePkgList"=> cfg.verbose_pkg_list = v.parse().unwrap_or(false),
                            "DisableDownloadTimeout"=> cfg.disable_download_timeout = v.parse().unwrap_or(false),
                            _=> {},                        
                        }
                    }
                },
                repo_name=> {
                    let mut repo = Repository::default();
                    repo.name = repo_name.to_string();
                    for (k, v) in prop.iter() {
                        match k.as_ref() {
                            "Server"=> repo.servers.push(v.to_string()),
                            "SigLevel"=> repo.sig_level.push(v.to_string()),
	                        "Usage" => repo.usage.push(v.to_string()),
                            _ => {},
                        }
                    }
                    cfg.repos.push(repo);
                },
            }
        }
        cfg
    }
    pub fn repository(&self, name: String) -> Option<&Repository>{
        self.repos.iter().find(|r| r.name == name)
    }
}
