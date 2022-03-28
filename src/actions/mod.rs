mod git;
mod registry;
mod template;

use std::{env::current_dir, fs};

use colored::Colorize;
pub use git::Git;
pub use registry::Registry;
pub use template::Template;
use walkdir::WalkDir;

fn read_ignore() {
    
}

pub fn check(keywords: &Vec<String>) -> std::io::Result<()> {
    if keywords.is_empty() {
        println!("{}", "请输入要检查的关键词".red());
        return Ok(());
    }
    for k in keywords {
        println!("check {}", k)
    }
    for entry in WalkDir::new(current_dir().unwrap()) {
        let entry = entry.unwrap();

        if let Some(full_name) = entry.path().to_str() {
            if full_name.contains(".git") {
                continue;
            }

            let file_type = fs::metadata(full_name)?.file_type();
            if file_type.is_dir() {
                continue;
            }
        }

        println!("{}", entry.path().display());
    }

    Ok(())
}
