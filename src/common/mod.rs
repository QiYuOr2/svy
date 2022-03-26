//! 通用处理

mod constants;
mod svy;
mod utils;

use constants::SVYRC;
use serde_json::Result;
use std::fs;
pub use svy::Svy;

/// 检查配置文件，没有则自动创建
pub fn check_config() -> Result<Svy> {
    if let Some(mut path) = dirs::home_dir() {
        path.push(SVYRC);
        // 不存在 -> 创建一个配置文件
        if !utils::file_exist(&path) {
            println!("{:?} 不存在", &path);
        }
        let content = fs::read_to_string(&path).unwrap();

        let config: Svy = serde_json::from_str(utils::string_to_static_str(content))?;
        return Ok(config);
    }
    panic!("$HOME获取失败")
}
