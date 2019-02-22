use crate::ini::Ini;

pub struct PacmanConfig {
    pub root_dir: String,
    pub db_path: String,
    pub cashe_dir: Vec<String>,
    pub hook_dir: Vec<String>,
    pub log_file: String,
    pub hold_pkg: Vec<String>,
    pub ignore_pkg: Vec<String>,
    pub ignore_group: Vec<String>,
    pub architecture: String,
    pub xfer_command: String,
    pub no_upgrade: Vec<String>,
    pub no_extract: Vec<String>,
    pub clean_method: Vec<String>,
    pub sig_level: Vec<String>,
    pub local_file_sig_level: Vec<String>,
    pub remote_file_sig_level: Vec<String>,
    pub use_syslog: bool,
    pub color: bool,
    pub use_delta: f64,
    pub total_download: bool,
    pub check_space:bool,
    pub verbose_pkg_list: bool,
    pub disable_download_timeout: bool,
    pub repos: Vec<Repository>,
}

pub struct Repository {
   pub name: String,
   pub servers: Vec<String>,
   pub sig_level: Vec<String>,
   pub usage: Vec<String>,
}

impl Default for Repository{
    fn default() -> Self{
        Repository{
            name: String::default(),
            servers: Vec::default(),
            sig_level: Vec::default(),
            usage: Vec::default(),
        }
    }
}

impl Default for PacmanConfig{
    fn default() -> Self{
        PacmanConfig{
            root_dir: Default::default(),
            db_path: Default::default(),
            cashe_dir: Default::default(),
            hook_dir: Default::default(),
            log_file: Default::default(),
            hold_pkg: Default::default(),
            ignore_pkg: Default::default(),
            ignore_group: Default::default(),
            architecture: Default::default(),
            xfer_command: Default::default(),
            no_upgrade: Default::default(),
            no_extract: Default::default(),
            clean_method: Default::default(),
            sig_level:    Default::default(),
            local_file_sig_level: Default::default(),
            remote_file_sig_level: Default::default(),
            use_syslog: Default::default(),
            color: Default::default(),
            use_delta:Default::default(),
            total_download: Default::default(),
            check_space:Default::default(),
            verbose_pkg_list: Default::default(),
            disable_download_timeout: Default::default(),
            repos: Default::default(),
        }
    }
}


impl PacmanConfig{
    pub fn parse(raw_cfg: &str) -> Self{
        let i = Ini::load_from_str(raw_cfg).unwrap_or_else(|_| Ini::new());
        let mut cfg = PacmanConfig::default();

        for (sec, prop) in i.iter() {
            let mut section : String;
            match sec{
                Some(s) => section = s.to_string(),
               _ => section = "".to_string(),
            }
            match section.as_str() {
                "options" => {
                    for (k, l_v) in prop.iter() {
                        for v in l_v.iter() {
                            match k.as_str() {
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
                                "UseSyslog"=> cfg.use_syslog = true,
                                "Color"=> cfg.color = v.parse().unwrap_or(false),
                                "UseDelta"=> cfg.use_delta = v.parse().unwrap_or(0.0),
                                "TotalDownload"=> cfg.total_download = v.parse().unwrap_or(false),
                                "CheckSpace"=> cfg.check_space = v.parse().unwrap_or(false),
                                "VerbosePkgList"=> cfg.verbose_pkg_list = v.parse().unwrap_or(false),
                                "DisableDownloadTimeout"=> cfg.disable_download_timeout = v.parse().unwrap_or(false),
                                _=> {},                        
                            }
                        }
                    }
                },
                repo_name=> {
                    let mut repo = Repository::default();
                    repo.name = repo_name.to_string();
                    for (k, l_v) in prop.iter() {
                        for v in l_v.iter() {
                            match k.as_ref() {
                                "Server"=> repo.servers.push(v.to_string()),
                                "SigLevel"=> repo.sig_level.push(v.to_string()),
                                "Usage" => repo.usage.push(v.to_string()),
                                _ => {},
                            }
                        }
                    }
                    cfg.repos.push(repo);
                },
            }
        }
        cfg
    }
    pub fn repo(&self, name: &str) -> Option<&Repository>{
        self.repos.iter().find(|r| r.name.as_str() == name)
    }
}
