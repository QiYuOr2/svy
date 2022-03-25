use clap::{Args, Parser, Subcommand};
// use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 切换npm源 svy registry [NAME] [--list|-l]
    Registry(Registry),

    /// 创建模板
    Create(Template),

    /// 修改Git配置
    Git(Git),

    /// 检查当前目录是否有敏感信息
    Check,
}

#[derive(Args)]
struct Registry {
    /// 查询所有源
    #[clap(short, long)]
    list: bool,

    /// 切换对应源
    name: Option<String>,
}

#[derive(Args)]
struct Git {}

#[derive(Args)]
struct Template {}

impl Registry {
    fn exec(&self) {
        // 含有`--list`时，查询所有源
        if self.list == true {
            println!("registry list");
            return;
        }
        // 切换源
        if let Some(name) = &self.name {
            println!("registry use {}", name)
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Registry(registry) => registry.exec(),
        Commands::Create(_) => {},
        Commands::Git(_) => {},
        Commands::Check => {}
    };
}
