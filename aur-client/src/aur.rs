use reqwest::{self,Url};

use serde::{Deserialize};
use git2::Repository;

use std::error::Error;
use std::path::Path;

static AUR_URL: &str = "https://aur.archlinux.org/";
static AUR_RPC_VER:&str = "5";

lazy_static!{
    static ref AUR_RPC_URL: String = Url::parse(AUR_URL).unwrap().join("/rpc").unwrap().to_string();
    static ref AUR_GIT_URL: Url = Url::parse(AUR_URL).unwrap();
}


#[derive(Deserialize)]
pub struct Package{
    pub ID : i32,
    pub Name : String,
    pub PackageBaseID : i32,
    pub PackageBase : Option<String>,
    pub Version : String,
    pub Description : Option<String>,
    pub URL : Option<String>,
    pub NumVotes : i32,
    pub Popularity : f64,
    pub OutOfDate : Option<i32>,
    pub Maintainer : Option<String>,
    pub FirstSubmitted : i32,
    pub LastModified : i32,
    pub URLPath : String,
    pub Depends : Option<Vec<String>>,
    pub MakeDepends : Option<Vec<String>>,
    pub CheckDepends : Option<Vec<String>>,
    pub Conflicts : Option<Vec<String>>,
    pub Provides : Option<Vec<String>>,
    pub Replaces : Option<Vec<String>>,
    pub OptDepends : Option<Vec<String>>,
    pub Groups : Option<Vec<String>>,
    pub License : Option<Vec<String>>,
    pub Keywords : Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Response {
    pub error : Option<String>,
    pub version: i32,
    pub r#type : String,
    pub resultcount : i32,
    pub results: Vec<Package>,
}

pub fn search(arg: &str) -> Result<Response, Box<Error>> {
    let url = Url::parse_with_params(&AUR_RPC_URL,
                                 &[("v", AUR_RPC_VER), ("type", "search"), ("arg", arg)])?;
    let resp: Response = reqwest::get(url.as_str())?
        .json()?;
    Ok(resp)
}

pub fn info(packages: &[&str]) -> Result<Response, Box<Error>>{
    let mut args = Vec::new();
    args.push(("v", AUR_RPC_VER));
    args.push(("type", "info"));
    for p in packages{
        args.push(("arg[]", p));
    }

    let url = Url::parse_with_params(&AUR_RPC_URL, &args)?;

    let resp: Response = reqwest::get(url.as_str())?
        .json()?;
    Ok(resp)
}

pub fn clone(package: &str, p: &Path) -> Result<Repository, Box<Error>>{
    Ok(Repository::clone(
        &AUR_GIT_URL.join(package)?
                    .join(".git")?
                    .to_string(),
        p
    )?)
}