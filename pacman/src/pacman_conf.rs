use crate::config::PacmanConfig;
use std::process::Command;

pub fn get_config() -> PacmanConfig{
    let raw_cfg = Command::new("pacman-conf")
        .output()
        .expect("failed to launch pacman-conf");

    let str_res = String::from_utf8_lossy(&raw_cfg.stdout);
    PacmanConfig::parse(&str_res)
}