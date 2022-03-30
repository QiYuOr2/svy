use std::{fs::File, path::Path};

use regex::Regex;

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
