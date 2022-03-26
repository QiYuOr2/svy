use std::{fs::File, path::Path};

pub fn file_exist<P: AsRef<Path>>(path: P) -> bool {
    let file = File::open(path);
    match file {
        Result::Err(_) => false,
        Result::Ok(_) => true,
    }
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
