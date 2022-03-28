//! 可执行子命令
use crate::actions::{Git, Registry, Template};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// 切换npm源 svy registry [NAME] [--list|-l]
    Registry(Registry),

    /// 创建模板
    Create(Template),

    /// 修改Git配置
    Git(Git),

    /// 检查当前目录是否有敏感信息
    /// 
    /// example: $ svy check password database
    Check { keywords: Vec<String> },
}
