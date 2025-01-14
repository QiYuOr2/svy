use std::{fs::File, path::Path, process::Command};

use regex::Regex;

use super::constants::GIT;

pub fn file_exist<P: AsRef<Path>>(path: P) -> bool {
    let file = File::open(path);
    match file {
        Result::Err(_) => false,
        Result::Ok(_) => true,
    }
}

pub fn regex_from_string(reg_str: &String) -> Option<Regex> {
    match Regex::new(reg_str) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

pub fn match_regex(regex: Option<Regex>, text: &String) -> bool {
    if let Some(re) = regex {
        return re.is_match(&text);
    }
    false
}

pub fn git_config(item: &str) -> String {
    String::from_utf8(
        Command::new(GIT)
            .arg("config")
            .arg(item)
            .output()
            .expect("git config 获取失败")
            .stdout,
    )
    .unwrap()
}

pub fn set_git_config(item: &str, value: &str) -> String {
    String::from_utf8(
        Command::new(GIT)
            .arg("config")
            .arg(item)
            .arg(value)
            .output()
            .expect("git config set失败")
            .stdout,
    )
    .unwrap()
}
