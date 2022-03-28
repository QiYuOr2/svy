//! 通用处理

mod constants;
mod svy;
mod utils;

use constants::SVYRC;
use serde_json::Result;
use std::{
    fs::{self, File},
    io::Write,
};
pub use svy::Svy;

const BASIC_CONFIG: &str = r#"{
    "registry": {
        "npm":"https://registry.npmjs.org/",
        "taobao":"https://registry.npmmirror.com/"
    },
    "git": {}
}"#;

/// 检查配置文件，没有则自动创建
pub fn check_config() -> Result<Svy> {
    if let Some(mut path) = dirs::home_dir() {
        path.push(SVYRC);
        // 不存在 -> 创建一个配置文件
        if !utils::file_exist(&path) {
            let mut config_file = match File::create(&path) {
                Err(error) => panic!("配置文件创建失败 {}: {}", &path.display(), error),
                Ok(file) => file,
            };

            match config_file.write_all(BASIC_CONFIG.as_bytes()) {
                Err(error) => panic!("配置文件写入失败 {}: {}", &path.display(), error),
                Ok(_) => println!("{:?} 已创建", &path),
            }
        }
        let content = fs::read_to_string(&path).unwrap();

        let config: Svy = serde_json::from_str(utils::string_to_static_str(content))?;
        return Ok(config);
    }
    panic!("$HOME获取失败")
}
