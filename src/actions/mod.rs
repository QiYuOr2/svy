mod git;
mod registry;
mod template;

use colored::Colorize;
use regex::Regex;
use std::{env::current_dir, fs, io::Result, str};
use walkdir::WalkDir;

pub use git::Git;
pub use registry::Registry;
pub use template::Template;

/// 读取当前目录下的.gitignore
fn read_ignore() -> Result<Vec<String>> {
    if let Ok(file) = fs::read(current_dir()?.join(".gitignore")) {
        let content = match str::from_utf8(&file) {
            Err(error) => panic!("读取ignore失败 {}", error),
            Ok(value) => value,
        };

        let mut result = vec![];

        for item in content.split_whitespace().collect::<Vec<&str>>() {
            result.push(String::from(item));
        }

        return Ok(result);
    }
    Ok(vec![])
}

fn check_file_content(path: &str, keywords: &Vec<String>) -> Result<bool> {
    let file = fs::read(path)?;

    let content = match str::from_utf8(&file) {
        Err(error) => panic!("读取 {} 失败 {}", path, error),
        Ok(value) => value,
    };

    Ok(keywords.iter().any(|k| content.contains(k)))
}

pub fn check(keywords: &Vec<String>) -> Result<()> {
    if keywords.is_empty() {
        println!("{}", "请输入要检查的关键词".red());
        return Ok(());
    }

    let mut ignore = read_ignore()?;
    ignore.push(".git".to_string());
    ignore.push(".DS_Store".to_string());

    let mut has_warn = false;

    for entry in WalkDir::new(current_dir().unwrap()) {
        let entry = entry.unwrap();

        if let Some(full_name) = entry.path().to_str() {
            // 过滤.gitignore中的文件
            let file_name = full_name
                .to_string()
                .replace(&current_dir()?.display().to_string(), "");

            if ignore
                .iter()
                .any(|i| Regex::new(i).unwrap().is_match(&file_name))
            {
                continue;
            }

            // 跳过目录
            let file_type = fs::metadata(full_name)?.file_type();
            if file_type.is_dir() {
                continue;
            }

            // 检查文件内容
            if check_file_content(&full_name, keywords)? {
                if !has_warn {
                    has_warn = true;
                }
                println!("[{}] 可能存在敏感数据，请检查", full_name.red())
            }
        }
    }

    if !has_warn {
        println!("{}", "检查通过！".green())
    }

    Ok(())
}
